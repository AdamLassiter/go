use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde_json::{Value, json};
use sqlx::Error;

use crate::{
    AppState,
    schema::{CreateLink, DeleteLink, FilterOptions, GetLink, UpdateLink},
    service::{create_link, delete_link, edit_link, get_link, get_links},
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

async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "GO API Services";

    let json_response = json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}

async fn get_links_handler(
    State(app_state): State<Arc<AppState>>,
    opts: Query<FilterOptions>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let (links, last) = get_links(&app_state, &opts).await.map_err(db_err)?;

    let json_response = json!({
        "paging": opts.into_paging(last, "/api/links", ""),
        "links": links
    });

    Ok(Json(json_response))
}

async fn create_link_handler(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateLink>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let link = create_link(&app_state, &body).await.map_err(db_err)?;

    let link_response = json!({
        "link": link,
    });

    Ok(Json(link_response))
}

async fn get_link_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let link = get_link(&app_state, &GetLink { id })
        .await
        .map_err(db_err)?;

    let link_response = json!({
        "link": link,
    });

    Ok(Json(link_response))
}

async fn edit_link_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateLink>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
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
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    delete_link(&app_state, &DeleteLink { id })
        .await
        .map_err(db_err)?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthcheck", get(health_check_handler))
        .route("/links", get(get_links_handler).post(create_link_handler))
        .route(
            "/links/{id}",
            get(get_link_handler)
                .put(edit_link_handler)
                .delete(delete_link_handler),
        )
        .with_state(app_state)
}
