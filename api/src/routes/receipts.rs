use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Extension, Multipart, Path, State},
    http::header,
    response::Response,
    routing::{get, post},
    Json, Router,
};
use chrono::DateTime;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::receipt::{CreateReceiptRequest, Receipt, ReceiptResponse, UpdateReceiptRequest};
use crate::storage::LocalStorage;

pub fn router(storage: Arc<LocalStorage>) -> Router<PgPool> {
    Router::new()
        .route("/", get(list_receipts).post(create_receipt))
        .route(
            "/{id}",
            get(get_receipt).put(update_receipt).delete(delete_receipt),
        )
        .route("/{id}/photo", post(upload_photo).get(get_photo))
        .layer(Extension(storage))
}

async fn build_response(pool: &PgPool, receipt: Receipt) -> Result<ReceiptResponse, AppError> {
    let categories = Receipt::get_categories(pool, receipt.id).await?;
    Ok(ReceiptResponse {
        id: receipt.id,
        user_id: receipt.user_id,
        card_id: receipt.card_id,
        total_amount: receipt.total_amount,
        photo_path: receipt.photo_path,
        notes: receipt.notes,
        purchased_at: receipt.purchased_at,
        created_at: receipt.created_at,
        categories: categories.into_iter().map(Into::into).collect(),
    })
}

async fn list_receipts(
    State(pool): State<PgPool>,
    AuthUser(_user): AuthUser,
) -> Result<Json<Vec<ReceiptResponse>>, AppError> {
    let receipts = Receipt::find_all(&pool).await?;
    let mut responses = Vec::with_capacity(receipts.len());
    for receipt in receipts {
        responses.push(build_response(&pool, receipt).await?);
    }
    Ok(Json(responses))
}

async fn create_receipt(
    State(pool): State<PgPool>,
    AuthUser(user): AuthUser,
    Json(req): Json<CreateReceiptRequest>,
) -> Result<Json<ReceiptResponse>, AppError> {
    let purchased_at = DateTime::parse_from_rfc3339(&req.purchased_at)
        .map_err(|e| AppError::BadRequest(format!("invalid date: {}", e)))?
        .with_timezone(&chrono::Utc);

    let receipt = Receipt::create(
        &pool,
        user.id,
        req.card_id,
        req.total_amount,
        None,
        req.notes.as_deref(),
        purchased_at,
    )
    .await?;

    if let Some(ref category_ids) = req.category_ids {
        Receipt::set_categories(&pool, receipt.id, category_ids).await?;
    }

    let response = build_response(&pool, receipt).await?;
    Ok(Json(response))
}

async fn get_receipt(
    State(pool): State<PgPool>,
    AuthUser(_user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ReceiptResponse>, AppError> {
    let receipt = Receipt::find_by_id(&pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    let response = build_response(&pool, receipt).await?;
    Ok(Json(response))
}

async fn update_receipt(
    State(pool): State<PgPool>,
    AuthUser(_user): AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateReceiptRequest>,
) -> Result<Json<ReceiptResponse>, AppError> {
    let purchased_at = req
        .purchased_at
        .as_ref()
        .map(|s| {
            DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .map_err(|e| AppError::BadRequest(format!("invalid date: {}", e)))
        })
        .transpose()?;

    let notes = req.notes.as_ref().map(|n| n.as_deref());

    let receipt = Receipt::update(
        &pool,
        id,
        req.card_id,
        req.total_amount,
        notes,
        purchased_at,
    )
    .await?
    .ok_or(AppError::NotFound)?;

    if let Some(ref category_ids) = req.category_ids {
        Receipt::set_categories(&pool, receipt.id, category_ids).await?;
    }

    let response = build_response(&pool, receipt).await?;
    Ok(Json(response))
}

async fn delete_receipt(
    State(pool): State<PgPool>,
    Extension(storage): Extension<Arc<LocalStorage>>,
    AuthUser(_user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    // Try to clean up the photo file if one exists
    if let Some(receipt) = Receipt::find_by_id(&pool, id).await? {
        if let Some(ref photo_path) = receipt.photo_path {
            let _ = storage.delete(photo_path).await;
        }
    }

    let deleted = Receipt::delete(&pool, id).await?;
    if !deleted {
        return Err(AppError::NotFound);
    }
    Ok(())
}

async fn upload_photo(
    State(pool): State<PgPool>,
    Extension(storage): Extension<Arc<LocalStorage>>,
    AuthUser(_user): AuthUser,
    Path(id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<ReceiptResponse>, AppError> {
    // Verify receipt exists
    let receipt = Receipt::find_by_id(&pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    // Get the file field
    let field = multipart
        .next_field()
        .await
        .map_err(|_| AppError::BadRequest("invalid multipart data".to_string()))?
        .ok_or(AppError::BadRequest("no file provided".to_string()))?;

    let content_type = field
        .content_type()
        .ok_or(AppError::BadRequest("missing content type".to_string()))?
        .to_string();
    let extension =
        LocalStorage::validate_content_type(&content_type).map_err(AppError::BadRequest)?;

    let data = field
        .bytes()
        .await
        .map_err(|_| AppError::BadRequest("failed to read file".to_string()))?;
    LocalStorage::validate_file_size(data.len()).map_err(AppError::BadRequest)?;

    let filename = LocalStorage::generate_filename(extension);
    storage
        .save(&filename, &data)
        .await
        .map_err(|_| AppError::Internal)?;

    // Delete old photo if exists
    if let Some(ref old_path) = receipt.photo_path {
        let _ = storage.delete(old_path).await;
    }

    let updated = Receipt::set_photo_path(&pool, id, &filename)
        .await?
        .ok_or(AppError::Internal)?;

    let response = build_response(&pool, updated).await?;
    Ok(Json(response))
}

async fn get_photo(
    State(pool): State<PgPool>,
    Extension(storage): Extension<Arc<LocalStorage>>,
    AuthUser(_user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
    let receipt = Receipt::find_by_id(&pool, id)
        .await?
        .ok_or(AppError::NotFound)?;
    let photo_path = receipt.photo_path.ok_or(AppError::NotFound)?;
    let file_path = storage.file_path(&photo_path);

    let content_type = if photo_path.ends_with(".png") {
        "image/png"
    } else {
        "image/jpeg"
    };

    let data = tokio::fs::read(&file_path)
        .await
        .map_err(|_| AppError::NotFound)?;

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .body(Body::from(data))
        .unwrap())
}
