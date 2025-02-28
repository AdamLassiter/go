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
    schema::{
        CreateLink, DeleteLink, FilterOptions, GetAllLinks, GetLink, SearchLinks, SearchMethod,
        SearchOptions, UpdateLink, ViewOptions,
    },
    service::{create_link, delete_link, edit_link, get_link, get_links, search_links},
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

pub async fn index_handler() -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    let template_response = LinksTemplate {}.render().map_err(tp_err)?;

    Ok(Html(template_response))
}

async fn get_links_handler(
    State(app_state): State<Arc<AppState>>,
    Query(filter): Query<FilterOptions>,
) -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    let get_all = GetAllLinks { filter };
    let (links, last) = get_links(&app_state, &get_all).await.map_err(db_err)?;
    let paging = filter.into_paging(last, "/go/links", "#links");

    let template_response = ListTemplate { links, paging }.render().map_err(tp_err)?;

    Ok(Html(template_response))
}

async fn search_links_handler(
    State(app_state): State<Arc<AppState>>,
    Path((method, query)): Path<(String, String)>,
    Query(filter): Query<FilterOptions>,
) -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    let path = format!("/go/links/{}/{}", method.as_str(), query);
    let method = SearchMethod::try_from_str(&method).ok_or((
        StatusCode::NOT_FOUND,
        Html(ErrorTemplate {}.render().map_err(tp_err)?),
    ))?;
    let search = SearchLinks {
        filter,
        search: SearchOptions { query, method },
    };
    let (links, last) = search_links(&app_state, &search).await.map_err(db_err)?;
    let paging = filter.into_paging(last, &path, "#links");

    let template_response = ListTemplate { links, paging }.render().map_err(tp_err)?;

    Ok(Html(template_response))
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
        .route("/links/{method}/{query}", get(search_links_handler))
        .route("/links", get(get_links_handler).post(create_link_handler))
        .route(
            "/link/{id}",
            get(get_link_handler)
                .delete(delete_link_handler)
                .put(edit_link_handler),
        )
        .with_state(app_state)
}
