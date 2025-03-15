use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use serde_json::{Value, json};
use sqlx::Error;

use crate::{
    AppState,
    model::Paging,
    schema::{
        CreateLink, DeleteLink, FindLink, GetLink, PagingOptions, QueryLinks, SearchOptions,
        UpdateLink,
    },
    service::{create_link, delete_link, edit_link, find_link, get_link, query_links},
};

fn db_err(err: Error) -> (StatusCode, Json<Value>) {
    match err {
        Error::Configuration(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": format!("Configuration error: {}", err)})),
        ),
        Error::PoolTimedOut => (
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({"message": format!("Pool timed out: {}", err)})),
        ),
        Error::RowNotFound | Error::ColumnNotFound(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({"message": format!("Not found: {}", err)})),
        ),
        Error::Encode(err)
        | Error::Decode(err)
        | Error::ColumnDecode {
            index: _,
            source: err,
        } => (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": format!("Failed to encode/decode: {}", err)})),
        ),
        Error::Io(err) => (
            StatusCode::INSUFFICIENT_STORAGE,
            Json(json!({"message": format!("Failed to read/write IO: {}", err)})),
        ),
        Error::Database(err) => {
            if err.message().contains("Duplicate entry") {
                let error_response = json!({
                    "message": format!("Already exists: {}", err),
                });
                (StatusCode::CONFLICT, Json(error_response))
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": format!("Database error: {}", err)})),
                )
            }
        }
        Error::Tls(_) => unreachable!("SQLite has no associated TLS layer"),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": format!("Database error: {}", err)})),
        ),
    }
}

async fn health_check_handler() -> Json<Value> {
    const MESSAGE: &str = "GO API Services";

    let json_response = json!({
        "message": MESSAGE
    });

    Json(json_response)
}

async fn query_links_handler(
    State(app_state): State<Arc<AppState>>,
    Query(paging): Query<PagingOptions>,
    Query(search): Query<SearchOptions>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let query = QueryLinks {
        paging,
        search: search.clone(),
    };
    let (links, last) = query_links(&app_state, &query).await.map_err(db_err)?;
    let paging = Paging::new(&paging, &search, last, "/api/links", "");

    let json_response = json!({
        "paging": paging,
        "links": links
    });

    Ok(Json(json_response))
}

async fn create_link_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateLink>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let link = create_link(&app_state, &body).await.map_err(db_err)?;

    let link_response = json!({
        "link": link,
    });

    Ok(Json(link_response))
}

async fn get_link_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let link = get_link(&app_state, &GetLink { id })
        .await
        .map_err(db_err)?;

    let link_response = json!({
        "link": link,
    });

    Ok(Json(link_response))
}

async fn find_link_handler(
    State(app_state): State<Arc<AppState>>,
    Query(search): Query<SearchOptions>,
    Query(paging): Query<PagingOptions>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let link = find_link(
        &app_state,
        &FindLink {
            source: search.query.clone(),
        },
    )
    .await
    .map_err(db_err)?;
    if let Some(link) = link {
        let link_response = json!({
            "link": link,
        });

        Ok(Json(link_response))
    } else {
        query_links_handler(State(app_state), Query(paging), Query(search)).await
    }
}

async fn edit_link_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateLink>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let updated_link = edit_link(&app_state, &GetLink { id }, &body)
        .await
        .map_err(db_err)?;

    let link_response = json!({
        "link": updated_link,
    });

    Ok(Json(link_response))
}

async fn delete_link_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    delete_link(&app_state, &DeleteLink { id })
        .await
        .map_err(db_err)?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthcheck", get(health_check_handler))
        .route("/links", get(query_links_handler).post(create_link_handler))
        .route(
            "/link/{id}",
            get(get_link_handler)
                .put(edit_link_handler)
                .delete(delete_link_handler),
        )
        .route("/search/{alias}", get(find_link_handler))
        .with_state(app_state)
}
