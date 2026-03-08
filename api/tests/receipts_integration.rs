//! Integration tests for receipt upload & management (Milestone 1.4).
//!
//! Requires a running PostgreSQL database.
//! Set DATABASE_URL environment variable before running.
//!
//! NOTE: The `reqwest` dev-dependency needs the `"multipart"` feature added
//! in Cargo.toml for the photo upload tests to compile.

use std::sync::Arc;

use axum::{routing::get, Json, Router};
use reqwest::multipart::{Form, Part};
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

    // Create temp storage directory for tests
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

fn new_client() -> Client {
    Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to build client")
}

fn unique_username() -> String {
    format!("testuser_{}", &Uuid::new_v4().to_string()[..8])
}

/// Register a new user and log them in. Returns the JSON response body.
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

/// Create a receipt and return the parsed JSON response.
async fn create_receipt(base_url: &str, client: &Client) -> Value {
    let res = client
        .post(format!("{}/api/receipts", base_url))
        .json(&json!({
            "total_amount": 1500,
            "purchased_at": "2026-03-07T00:00:00Z",
            "card_id": null,
            "notes": "integration test receipt",
            "category_ids": []
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    res.json().await.unwrap()
}

/// Build a minimal fake JPEG byte sequence for upload tests.
fn fake_jpeg_bytes() -> Vec<u8> {
    // JPEG SOI marker + APP0 JFIF header (minimal)
    vec![
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00,
    ]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_receipt_crud_lifecycle() {
    let (base_url, client) = spawn_app().await;
    register_and_login(&base_url, &client).await;

    // --- Create a receipt ---
    let res = client
        .post(format!("{}/api/receipts", base_url))
        .json(&json!({
            "total_amount": 1500,
            "purchased_at": "2026-03-07T00:00:00Z",
            "card_id": null,
            "notes": "grocery run",
            "category_ids": []
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let receipt: Value = res.json().await.unwrap();
    assert_eq!(receipt["total_amount"], 1500);
    assert_eq!(receipt["notes"], "grocery run");
    let receipt_id = receipt["id"].as_str().expect("receipt id should be a string");

    // --- List receipts — should contain the new receipt ---
    let res = client
        .get(format!("{}/api/receipts", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let receipts: Vec<Value> = res.json().await.unwrap();
    assert!(
        receipts.iter().any(|r| r["id"].as_str() == Some(receipt_id)),
        "Created receipt should appear in list"
    );

    // --- Get receipt by id ---
    let res = client
        .get(format!("{}/api/receipts/{}", base_url, receipt_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let fetched: Value = res.json().await.unwrap();
    assert_eq!(fetched["total_amount"], 1500);
    assert_eq!(fetched["notes"], "grocery run");
    assert_eq!(fetched["purchased_at"], "2026-03-07T00:00:00Z");

    // --- Update the receipt ---
    let res = client
        .put(format!("{}/api/receipts/{}", base_url, receipt_id))
        .json(&json!({
            "total_amount": 2500,
            "purchased_at": "2026-03-07T00:00:00Z",
            "card_id": null,
            "notes": "updated notes",
            "category_ids": []
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let updated: Value = res.json().await.unwrap();
    assert_eq!(updated["total_amount"], 2500);
    assert_eq!(updated["notes"], "updated notes");

    // --- Verify update via GET ---
    let res = client
        .get(format!("{}/api/receipts/{}", base_url, receipt_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let verified: Value = res.json().await.unwrap();
    assert_eq!(verified["total_amount"], 2500);
    assert_eq!(verified["notes"], "updated notes");

    // --- Delete the receipt ---
    let res = client
        .delete(format!("{}/api/receipts/{}", base_url, receipt_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    // --- Verify deletion — GET should return 404 ---
    let res = client
        .get(format!("{}/api/receipts/{}", base_url, receipt_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 404);

    // --- List should no longer contain the receipt ---
    let res = client
        .get(format!("{}/api/receipts", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let receipts: Vec<Value> = res.json().await.unwrap();
    assert!(
        !receipts.iter().any(|r| r["id"].as_str() == Some(receipt_id)),
        "Deleted receipt should not appear in list"
    );
}

#[tokio::test]
async fn test_receipt_with_photo_upload() {
    let (base_url, client) = spawn_app().await;
    register_and_login(&base_url, &client).await;

    // Create a receipt
    let receipt = create_receipt(&base_url, &client).await;
    let receipt_id = receipt["id"].as_str().expect("receipt id should be a string");

    // Upload a photo
    let form = Form::new().part(
        "file",
        Part::bytes(fake_jpeg_bytes())
            .file_name("test.jpg")
            .mime_str("image/jpeg")
            .unwrap(),
    );
    let res = client
        .post(format!("{}/api/receipts/{}/photo", base_url, receipt_id))
        .multipart(form)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let updated: Value = res.json().await.unwrap();
    assert!(
        updated["photo_path"].as_str().is_some(),
        "photo_path should be set after upload"
    );
    assert!(
        !updated["photo_path"].as_str().unwrap().is_empty(),
        "photo_path should not be empty"
    );

    // Get receipt — verify photo_path is set
    let res = client
        .get(format!("{}/api/receipts/{}", base_url, receipt_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let fetched: Value = res.json().await.unwrap();
    assert!(fetched["photo_path"].as_str().is_some());

    // Get photo — verify returns 200 with image content type
    let res = client
        .get(format!("{}/api/receipts/{}/photo", base_url, receipt_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let content_type = res
        .headers()
        .get("content-type")
        .expect("response should have content-type header")
        .to_str()
        .unwrap()
        .to_string();
    assert!(
        content_type.starts_with("image/"),
        "Content-Type should be an image type, got: {}",
        content_type
    );
    let body = res.bytes().await.unwrap();
    assert!(!body.is_empty(), "Photo response body should not be empty");
}

#[tokio::test]
async fn test_photo_upload_invalid_type_rejected() {
    let (base_url, client) = spawn_app().await;
    register_and_login(&base_url, &client).await;

    // Create a receipt
    let receipt = create_receipt(&base_url, &client).await;
    let receipt_id = receipt["id"].as_str().expect("receipt id should be a string");

    // Try uploading a text file as text/plain — should be rejected
    let form = Form::new().part(
        "file",
        Part::bytes(b"this is not an image".to_vec())
            .file_name("notes.txt")
            .mime_str("text/plain")
            .unwrap(),
    );
    let res = client
        .post(format!("{}/api/receipts/{}/photo", base_url, receipt_id))
        .multipart(form)
        .send()
        .await
        .unwrap();
    assert_eq!(
        res.status(),
        400,
        "Uploading a text/plain file should return 400"
    );
}

#[tokio::test]
async fn test_unauthenticated_photo_retrieval_returns_401() {
    let (base_url, client) = spawn_app().await;
    register_and_login(&base_url, &client).await;

    // Create a receipt and upload a photo
    let receipt = create_receipt(&base_url, &client).await;
    let receipt_id = receipt["id"].as_str().expect("receipt id should be a string");

    let form = Form::new().part(
        "file",
        Part::bytes(fake_jpeg_bytes())
            .file_name("test.jpg")
            .mime_str("image/jpeg")
            .unwrap(),
    );
    let res = client
        .post(format!("{}/api/receipts/{}/photo", base_url, receipt_id))
        .multipart(form)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    // New client (no cookies / unauthenticated) tries to GET the photo
    let anon_client = new_client();
    let res = anon_client
        .get(format!("{}/api/receipts/{}/photo", base_url, receipt_id))
        .send()
        .await
        .unwrap();
    assert_eq!(
        res.status(),
        401,
        "Unauthenticated photo retrieval should return 401"
    );
}

#[tokio::test]
async fn test_category_crud_and_receipt_assignment() {
    let (base_url, client) = spawn_app().await;
    register_and_login(&base_url, &client).await;

    // --- Create two categories ---
    let res = client
        .post(format!("{}/api/categories", base_url))
        .json(&json!({
            "name": "Groceries",
            "color": "#00aa00"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let cat1: Value = res.json().await.unwrap();
    let cat1_id = cat1["id"].as_str().expect("category id should be a string");

    let res = client
        .post(format!("{}/api/categories", base_url))
        .json(&json!({
            "name": "Dining",
            "color": "#ff5500"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let cat2: Value = res.json().await.unwrap();
    let cat2_id = cat2["id"].as_str().expect("category id should be a string");

    // --- Create a receipt with both category_ids ---
    let res = client
        .post(format!("{}/api/receipts", base_url))
        .json(&json!({
            "total_amount": 3200,
            "purchased_at": "2026-03-07T00:00:00Z",
            "card_id": null,
            "notes": "dinner and snacks",
            "category_ids": [cat1_id, cat2_id]
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let receipt: Value = res.json().await.unwrap();
    let receipt_id = receipt["id"].as_str().expect("receipt id should be a string");

    // Verify categories are included in receipt response
    let categories = receipt["categories"]
        .as_array()
        .expect("categories should be an array");
    assert_eq!(
        categories.len(),
        2,
        "Receipt should have 2 categories assigned"
    );
    let cat_ids: Vec<&str> = categories
        .iter()
        .map(|c| c["id"].as_str().unwrap())
        .collect();
    assert!(cat_ids.contains(&cat1_id));
    assert!(cat_ids.contains(&cat2_id));

    // --- List categories — verify both exist ---
    let res = client
        .get(format!("{}/api/categories", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let all_cats: Vec<Value> = res.json().await.unwrap();
    assert!(
        all_cats.iter().any(|c| c["id"].as_str() == Some(cat1_id)),
        "Groceries category should be in list"
    );
    assert!(
        all_cats.iter().any(|c| c["id"].as_str() == Some(cat2_id)),
        "Dining category should be in list"
    );

    // --- Delete one category ---
    let res = client
        .delete(format!("{}/api/categories/{}", base_url, cat1_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    // --- Get receipt again — verify only one category remains ---
    let res = client
        .get(format!("{}/api/receipts/{}", base_url, receipt_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let receipt: Value = res.json().await.unwrap();
    let categories = receipt["categories"]
        .as_array()
        .expect("categories should be an array");
    assert_eq!(
        categories.len(),
        1,
        "Receipt should have 1 category after deletion"
    );
    assert_eq!(categories[0]["id"].as_str().unwrap(), cat2_id);
}

#[tokio::test]
async fn test_receipts_shared_between_users() {
    let (base_url, client_a) = spawn_app().await;

    // Register user A and create a receipt
    register_and_login(&base_url, &client_a).await;
    let receipt = create_receipt(&base_url, &client_a).await;
    let receipt_id = receipt["id"].as_str().expect("receipt id should be a string");

    // Register user B with a separate client (separate cookie jar)
    let client_b = new_client();
    register_and_login(&base_url, &client_b).await;

    // User B lists receipts — should see user A's receipt (shared household)
    let res = client_b
        .get(format!("{}/api/receipts", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let receipts: Vec<Value> = res.json().await.unwrap();
    assert!(
        receipts
            .iter()
            .any(|r| r["id"].as_str() == Some(receipt_id)),
        "User B should see User A's receipt (receipts are shared)"
    );
}

#[tokio::test]
async fn test_unauthenticated_receipt_access_returns_401() {
    let (base_url, client) = spawn_app().await;

    // GET /api/receipts without login
    let res = client
        .get(format!("{}/api/receipts", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);

    // POST /api/receipts without login
    let res = client
        .post(format!("{}/api/receipts", base_url))
        .json(&json!({
            "total_amount": 1000,
            "purchased_at": "2026-03-07T00:00:00Z"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
}

#[tokio::test]
async fn test_category_crud_lifecycle() {
    let (base_url, client) = spawn_app().await;
    register_and_login(&base_url, &client).await;

    // Create a category
    let res = client
        .post(format!("{}/api/categories", base_url))
        .json(&json!({
            "name": "Electronics",
            "color": "#3b82f6"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let cat: Value = res.json().await.unwrap();
    assert_eq!(cat["name"], "Electronics");
    assert_eq!(cat["color"], "#3b82f6");
    let cat_id = cat["id"].as_str().expect("category id");

    // List categories
    let res = client
        .get(format!("{}/api/categories", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let cats: Vec<Value> = res.json().await.unwrap();
    assert!(cats.iter().any(|c| c["id"].as_str() == Some(cat_id)));

    // Get category by id
    let res = client
        .get(format!("{}/api/categories/{}", base_url, cat_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let fetched: Value = res.json().await.unwrap();
    assert_eq!(fetched["name"], "Electronics");

    // Update category
    let res = client
        .put(format!("{}/api/categories/{}", base_url, cat_id))
        .json(&json!({
            "name": "Tech",
            "color": "#ef4444"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let updated: Value = res.json().await.unwrap();
    assert_eq!(updated["name"], "Tech");
    assert_eq!(updated["color"], "#ef4444");

    // Delete category
    let res = client
        .delete(format!("{}/api/categories/{}", base_url, cat_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    // Verify deleted
    let res = client
        .get(format!("{}/api/categories/{}", base_url, cat_id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 404);
}
