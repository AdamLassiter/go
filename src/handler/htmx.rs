use std::sync::Arc;

use askama::Template;
use axum::{
    Form, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};

use crate::{
    AppState,
    model::Paging,
    schema::{
        CreateLink, DeleteLink, FindLink, GetLink, PagingOptions, QueryLinks, SearchOptions,
        UpdateLink, ViewOptions,
    },
    service::{create_link, delete_link, edit_link, find_link, get_link, query_links},
    template::{EditTemplate, ErrorTemplate, LinksTemplate, ListTemplate, ViewTemplate},
};

fn db_err(err: sqlx::Error) -> (StatusCode, Html<String>) {
    eprintln!("{}", err);
    (
        StatusCode::BAD_REQUEST,
        Html(ErrorTemplate {}.render().unwrap_or("Oops!".to_string())),
    )
}

fn tp_err(err: askama::Error) -> (StatusCode, Html<String>) {
    eprintln!("{}", err);
    (
        StatusCode::BAD_REQUEST,
        Html(ErrorTemplate {}.render().unwrap_or("Oops!".to_string())),
    )
}

pub async fn index_handler(
    Query(paging): Query<PagingOptions>,
    Query(search): Query<SearchOptions>,
) -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    let template_response = LinksTemplate { paging, search }.render().map_err(tp_err)?;

    Ok(Html(template_response))
}

async fn query_links_handler(
    State(app_state): State<Arc<AppState>>,
    Query(paging): Query<PagingOptions>,
    Query(search): Query<SearchOptions>,
) -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    let find = FindLink {
        source: search.query.clone(),
    };
    let link = find_link(&app_state, &find).await.map_err(db_err)?;
    let new = match link {
        None if !find.source.is_empty() => Some(CreateLink {
            source: find.source,
            is_alias: false,
            target: "".to_string(),
        }),
        _ => None,
    };

    let query = QueryLinks {
        paging,
        search: search.clone(),
    };
    let (links, last) = query_links(&app_state, &query).await.map_err(db_err)?;

    let paging = Paging::new(&paging, &search, last, "/go/links", "#links");
    let hx_push_url = paging.full_query();

    let template_response = ListTemplate { new, links, paging }
        .render()
        .map_err(tp_err)?;

    Ok(([("HX-Push-Url", hx_push_url)], Html(template_response)))
}

async fn create_link_handler(
    State(app_state): State<Arc<AppState>>,
    Form(body): Form<CreateLink>,
) -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    let link = create_link(&app_state, &body).await.map_err(db_err)?;

    let template_response = ViewTemplate { link }.render().map_err(tp_err)?;

    Ok(Html(template_response))
}

async fn edit_link_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Form(body): Form<UpdateLink>,
) -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    let link = edit_link(&app_state, &GetLink { id }, &body)
        .await
        .map_err(db_err)?;

    let template_response = ViewTemplate { link }.render().map_err(tp_err)?;

    Ok(Html(template_response))
}

async fn get_link_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    view: Query<ViewOptions>,
) -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    let link = get_link(&app_state, &GetLink { id })
        .await
        .map_err(db_err)?;

    let template_response = if view.editable {
        EditTemplate { link }.render()
    } else {
        ViewTemplate { link }.render()
    }
    .map_err(tp_err)?;

    Ok(Html(template_response))
}

async fn delete_link_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    delete_link(&app_state, &DeleteLink { id })
        .await
        .map_err(db_err)?;

    Ok(Html(()))
}

pub fn router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/links", get(query_links_handler).post(create_link_handler))
        .route(
            "/link/{id}",
            get(get_link_handler)
                .delete(delete_link_handler)
                .put(edit_link_handler),
        )
        .with_state(app_state)
}
