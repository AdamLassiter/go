use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::{
    handler::{api, htmx::{self, index_handler}}, AppState
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/api", api::router(app_state.clone()))
        .nest_service("/go", htmx::router(app_state.clone()))
        .route("/", get(index_handler))
        .with_state(app_state)
}
