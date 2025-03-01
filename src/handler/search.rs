use std::sync::Arc;

use askama::Template;
use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::get,
};

use crate::{
    AppState,
    schema::{FindLink, SearchOptions},
    service::find_link,
    template::ErrorTemplate,
};

fn db_err(err: sqlx::Error) -> (StatusCode, Html<String>) {
    eprintln!("{}", err);
    (
        StatusCode::BAD_REQUEST,
        Html(ErrorTemplate {}.render().unwrap_or("Oops!".to_string())),
    )
}

async fn find_link_handler(
    State(app_state): State<Arc<AppState>>,
    Query(search): Query<SearchOptions>,
) -> Result<impl IntoResponse, (StatusCode, Html<String>)> {
    let link = find_link(&app_state, &FindLink {
        source: search.query.clone(),
    })
    .await
    .map_err(db_err)?;

    if let Some(link) = link {
        Ok(Redirect::to(&link.target))
    } else {
        let path = format!("/?query={}&method={}", search.query, search.method.as_str());
        Ok(Redirect::to(&path))
    }
}

pub fn router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(find_link_handler))
        .with_state(app_state)
}
