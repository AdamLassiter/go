mod handler;
mod model;
mod query;
mod route;
mod schema;
mod service;
mod template;

use std::{env, sync::Arc};

use axum::http::{Method, header::CONTENT_TYPE};

use dotenv::dotenv;
use tokio::net::TcpListener;

use sqlx::{
    Error, Executor, Pool, Sqlite, SqlitePool,
    migrate::MigrateError,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use route::create_router;
use tower_http::cors::{Any, CorsLayer};

pub struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    println!("🔎 Go! Crowdsourced Search Service");

    let db = init_db().await?;

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

async fn init_db() -> Result<Pool<Sqlite>, Error> {
    let conn_opts = SqliteConnectOptions::new()
        .filename(env::var("DATABASE_FILENAME").expect("DATABASE_FILENAME not set"))
        .extension("extensions/vec")
        .extension("extensions/lembed")
        .extension("extensions/fuzzy")
        .create_if_missing(true);

    let pool_opts = SqlitePoolOptions::new().after_connect(|conn, _meta| {
        Box::pin(async move {
            println!("🔄 Running initialisations...");
            let initialiser = sqlx::migrate!("./initialisations");
            for migration in initialiser
                .iter()
                .filter(|&initialisation| initialisation.migration_type.is_up_migration())
            {
                conn.execute(&*migration.sql)
                    .await
                    .map_err(MigrateError::Execute)?;
            }
            Ok(())
        })
    });

    let db = pool_opts.connect_with(conn_opts).await?;

    println!("🔄 Running migrations...");
    sqlx::migrate!("./migrations").run(&db).await?;

    Ok(db)
}
