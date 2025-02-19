#![feature(int_roundings)]

mod handler;
mod model;
mod route;
mod schema;
mod service;
mod template;

use std::{env, sync::Arc};

use axum::http::{Method, header::CONTENT_TYPE};

use dotenv::dotenv;
use tokio::net::TcpListener;

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

use route::create_router;
use tower_http::cors::{Any, CorsLayer};

pub struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    println!("🔎 Go! Crowdsourced Search Service");

    let options = SqliteConnectOptions::new()
        .filename(env::var("DATABASE_FILENAME").expect("DATABASE_FILENAME not set"))
        .create_if_missing(true);

    // let pool = sqlx::sqlite::SqlitePool::connect("sqlite:go.db").await?;
    let db = SqlitePool::connect_with(options).await?;
    sqlx::migrate!().run(&db).await?;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db })).layer(cors);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("✅ Server started successfully at 0.0.0.0:8080");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
