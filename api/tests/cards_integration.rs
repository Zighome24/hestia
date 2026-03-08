use std::sync::Arc;

use axum::{routing::get, Json, Router};
use reqwest::Client;
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;
use uuid::Uuid;

use hestia_api::routes;
use hestia_api::storage::LocalStorage;

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

    let storage_path = format!("/tmp/hestia_test_{}", Uuid::new_v4());
    let storage = Arc::new(LocalStorage::new(&storage_path));
    storage.init().await.unwrap();

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
        .nest("/api/receipts", routes::receipts::router(storage))
        .nest("/api/cards", routes::cards::router())
        .nest("/api/categories", routes::categories::router())
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

/// Spawn an app instance with a fresh client (separate cookie jar).
/// Reuses the same server address.
fn new_client() -> Client {
    Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to build client")
}

fn unique_username() -> String {
    format!("testuser_{}", &Uuid::new_v4().to_string()[..8])
}

/// Register a new user and log them in. Returns the JSON response body from registration.
/// The provided client will hold the session cookie afterward.
async fn register_and_login(base_url: &str, client: &Client) -> Value {
    let username = unique_username();
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
    res.json().await.unwrap()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_card_crud_lifecycle() {
    let (base_url, client) = spawn_app().await;
    register_and_login(&base_url, &client).await;

    // --- Create a card ---
    let res = client
        .post(format!("{}/api/cards", base_url))
        .json(&json!({
            "nickname": "My Visa",
            "last_four": "4321"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let card: Value = res.json().await.unwrap();
    assert_eq!(card["nickname"], "My Visa");
    assert_eq!(card["last_four"], "4321");
    let card_id = card["id"].as_str().expect("card id should be a string");

    // --- List cards — should contain the new card ---
    let res = client
        .get(format!("{}/api/cards", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let cards: Vec<Value> = res.json().await.unwrap();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0]["id"].as_str().unwrap(), card_id);

    // --- Get card by id ---
    let res = client
        .get(format!("{}/api/cards/{}", base_url, card_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let fetched: Value = res.json().await.unwrap();
    assert_eq!(fetched["nickname"], "My Visa");
    assert_eq!(fetched["last_four"], "4321");

    // --- Update the card ---
    let res = client
        .put(format!("{}/api/cards/{}", base_url, card_id))
        .json(&json!({
            "nickname": "Work Amex",
            "last_four": "9999"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let updated: Value = res.json().await.unwrap();
    assert_eq!(updated["nickname"], "Work Amex");
    assert_eq!(updated["last_four"], "9999");

    // --- Verify update via GET ---
    let res = client
        .get(format!("{}/api/cards/{}", base_url, card_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let verified: Value = res.json().await.unwrap();
    assert_eq!(verified["nickname"], "Work Amex");
    assert_eq!(verified["last_four"], "9999");

    // --- Delete the card ---
    let res = client
        .delete(format!("{}/api/cards/{}", base_url, card_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    // --- Verify deletion — list should be empty ---
    let res = client
        .get(format!("{}/api/cards", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let cards: Vec<Value> = res.json().await.unwrap();
    assert!(cards.is_empty(), "Card list should be empty after deletion");
}

#[tokio::test]
async fn test_unauthenticated_card_access_returns_401() {
    let (base_url, client) = spawn_app().await;
    // Do NOT register or log in — client has no session.

    // GET /api/cards
    let res = client
        .get(format!("{}/api/cards", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);

    // POST /api/cards
    let res = client
        .post(format!("{}/api/cards", base_url))
        .json(&json!({
            "nickname": "Sneaky Card",
            "last_four": "0000"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
}

#[tokio::test]
async fn test_cards_scoped_to_user() {
    let (base_url, client_a) = spawn_app().await;
    let client_b = new_client();

    // Register and login User A
    register_and_login(&base_url, &client_a).await;

    // User A creates a card
    let res = client_a
        .post(format!("{}/api/cards", base_url))
        .json(&json!({
            "nickname": "A's Card",
            "last_four": "1111"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let card_a: Value = res.json().await.unwrap();
    let card_a_id = card_a["id"].as_str().expect("card id should be a string");

    // Register and login User B (separate client / cookie jar)
    register_and_login(&base_url, &client_b).await;

    // User B lists cards — should be empty
    let res = client_b
        .get(format!("{}/api/cards", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let cards_b: Vec<Value> = res.json().await.unwrap();
    assert!(
        cards_b.is_empty(),
        "User B should not see User A's cards"
    );

    // User B tries to GET User A's card by ID — should get 404
    let res = client_b
        .get(format!("{}/api/cards/{}", base_url, card_a_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 404);

    // User B tries to DELETE User A's card — should get 404
    let res = client_b
        .delete(format!("{}/api/cards/{}", base_url, card_a_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 404);
}

#[tokio::test]
async fn test_card_validation_errors() {
    let (base_url, client) = spawn_app().await;
    register_and_login(&base_url, &client).await;

    // last_four too short (3 digits)
    let res = client
        .post(format!("{}/api/cards", base_url))
        .json(&json!({
            "nickname": "Bad Card",
            "last_four": "123"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);

    // last_four contains non-digits
    let res = client
        .post(format!("{}/api/cards", base_url))
        .json(&json!({
            "nickname": "Bad Card",
            "last_four": "ab12"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);

    // Empty nickname
    let res = client
        .post(format!("{}/api/cards", base_url))
        .json(&json!({
            "nickname": "",
            "last_four": "5678"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);
}
