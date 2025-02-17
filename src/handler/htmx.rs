use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, Query, State}, http::StatusCode, response::{Html, IntoResponse}, routing::get, Form, Router
};

use crate::{
    AppState,
    schema::{CreateLink, FilterOptions, GetLink},
    service::{create_link, get_link, get_links},
    template::{ErrorTemplate, IndexTemplate, LinkTemplate, LinksTemplate},
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

pub async fn index_handler() -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    let template_response = IndexTemplate {}.render().map_err(tp_err)?;

    Ok(Html(template_response))
}

async fn get_links_handler(
    State(app_state): State<Arc<AppState>>,
    opts: Query<FilterOptions>,
) -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    let links = get_links(&app_state, &opts).await.map_err(db_err)?;

    let template_response = LinksTemplate { links }.render().map_err(tp_err)?;

    Ok(Html(template_response))
}

async fn create_link_handler(
    State(app_state): State<Arc<AppState>>,
    Form(body): Form<CreateLink>,
) -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    let link = create_link(&app_state, &body).await.map_err(db_err)?;

    let template_response = LinkTemplate { link }.render().map_err(tp_err)?;

    Ok(Html(template_response))
}

async fn get_link_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    let link = get_link(&app_state, &GetLink { id })
        .await
        .map_err(db_err)?;

    let template_response = LinkTemplate { link }.render().map_err(tp_err)?;

    Ok(Html(template_response))
}

pub fn router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/links", get(get_links_handler).post(create_link_handler))
        .route("/links/{id}", get(get_link_handler))
        .with_state(app_state)
}
