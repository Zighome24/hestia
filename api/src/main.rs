mod auth;
mod config;
mod error;
mod models;
mod routes;
mod storage;

use axum::{
    routing::get,
    Json, Router,
};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hestia_api=debug,tower_http=debug".into()),
        )
        .init();

    let config = config::Config::from_env()?;

    info!("connecting to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    info!("running migrations");
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/api/health", get(health_handler))
        .nest("/api/auth", routes::auth::router())
        .nest("/api/receipts", routes::receipts::router())
        .nest("/api/cards", routes::cards::router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = format!("0.0.0.0:{}", config.port);
    info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}
