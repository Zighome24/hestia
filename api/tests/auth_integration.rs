//! Integration tests for authentication flows.
//!
//! Requires a running PostgreSQL database.
//! Set DATABASE_URL and SESSION_SECRET environment variables before running.

use axum::{routing::get, Json, Router};
use reqwest::Client;
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;
use uuid::Uuid;

use hestia_api::routes;

async fn spawn_app() -> (String, Client) {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let session_store = PostgresStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .expect("Failed to migrate session store");

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_http_only(true);

    let app = Router::new()
        .route(
            "/api/health",
            get(|| async { Json(json!({"status": "ok"})) }),
        )
        .nest("/api/auth", routes::auth::router())
        .nest("/api/receipts", routes::receipts::router())
        .nest("/api/cards", routes::cards::router())
        .layer(session_layer)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind");
    let addr = listener.local_addr().expect("Failed to get local addr");

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to build client");

    (format!("http://{}", addr), client)
}

fn unique_username() -> String {
    format!("testuser_{}", &Uuid::new_v4().to_string()[..8])
}

#[tokio::test]
async fn test_register_login_me_logout_flow() {
    let (base_url, client) = spawn_app().await;
    let username = unique_username();

    // Register
    let res = client
        .post(format!("{}/api/auth/register", base_url))
        .json(&json!({
            "username": username,
            "display_name": "Test User",
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["username"], username);
    assert!(body.get("id").is_some());

    // After register, should be auto-logged in
    let res = client
        .get(format!("{}/api/auth/me", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let body: Value = res.json().await.unwrap();
    assert_eq!(body["username"], username);

    // Logout
    let res = client
        .post(format!("{}/api/auth/logout", base_url))
        .json(&json!({}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    // After logout, /me should return 401
    let res = client
        .get(format!("{}/api/auth/me", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);

    // Login again
    let res = client
        .post(format!("{}/api/auth/login", base_url))
        .json(&json!({
            "username": username,
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    // Should be logged in again
    let res = client
        .get(format!("{}/api/auth/me", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn test_unauthenticated_me_returns_401() {
    let (base_url, client) = spawn_app().await;

    let res = client
        .get(format!("{}/api/auth/me", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
}

#[tokio::test]
async fn test_login_wrong_password_returns_401() {
    let (base_url, client) = spawn_app().await;
    let username = unique_username();

    // Register
    client
        .post(format!("{}/api/auth/register", base_url))
        .json(&json!({
            "username": username,
            "display_name": "Test User",
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();

    // Logout
    client
        .post(format!("{}/api/auth/logout", base_url))
        .json(&json!({}))
        .send()
        .await
        .unwrap();

    // Try login with wrong password
    let res = client
        .post(format!("{}/api/auth/login", base_url))
        .json(&json!({
            "username": username,
            "password": "wrongpassword"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
}

#[tokio::test]
async fn test_duplicate_registration_fails() {
    let (base_url, client) = spawn_app().await;
    let username = unique_username();

    // Register first time
    let res = client
        .post(format!("{}/api/auth/register", base_url))
        .json(&json!({
            "username": username,
            "display_name": "Test User",
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    // Try duplicate registration with a fresh client
    let client2 = Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    let res = client2
        .post(format!("{}/api/auth/register", base_url))
        .json(&json!({
            "username": username,
            "display_name": "Another User",
            "password": "password456"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
}
