use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;

use crate::{
    AppState,
    handler::{
        api,
        htmx::{self, index_handler},
    },
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest_service("/api", api::router(app_state.clone()))
        .nest_service("/go", htmx::router(app_state.clone()))
        .route("/", get(index_handler))
        .fallback_service(ServeDir::new("static"))
        .with_state(app_state)
}
