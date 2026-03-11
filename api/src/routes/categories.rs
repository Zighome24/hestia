use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::category::{Category, CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/", get(list_categories).post(create_category))
        .route(
            "/:id",
            get(get_category).put(update_category).delete(delete_category),
        )
}

async fn list_categories(
    State(pool): State<PgPool>,
    AuthUser(_user): AuthUser,
) -> Result<Json<Vec<CategoryResponse>>, AppError> {
    let categories = Category::find_all(&pool).await?;
    Ok(Json(categories.into_iter().map(Into::into).collect()))
}

async fn create_category(
    State(pool): State<PgPool>,
    AuthUser(_user): AuthUser,
    Json(req): Json<CreateCategoryRequest>,
) -> Result<Json<CategoryResponse>, AppError> {
    Category::validate_name(&req.name).map_err(AppError::BadRequest)?;

    let color = req.color.as_deref().unwrap_or("#6b7280");
    Category::validate_color(color).map_err(AppError::BadRequest)?;

    let category = Category::create(&pool, &req.name, color).await?;
    Ok(Json(category.into()))
}

async fn get_category(
    State(pool): State<PgPool>,
    AuthUser(_user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<CategoryResponse>, AppError> {
    let category = Category::find_by_id(&pool, id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(category.into()))
}

async fn update_category(
    State(pool): State<PgPool>,
    AuthUser(_user): AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCategoryRequest>,
) -> Result<Json<CategoryResponse>, AppError> {
    if let Some(ref name) = req.name {
        Category::validate_name(name).map_err(AppError::BadRequest)?;
    }
    if let Some(ref color) = req.color {
        Category::validate_color(color).map_err(AppError::BadRequest)?;
    }

    let category = Category::update(&pool, id, req.name.as_deref(), req.color.as_deref())
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(category.into()))
}

async fn delete_category(
    State(pool): State<PgPool>,
    AuthUser(_user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    let deleted = Category::delete(&pool, id).await?;
    if !deleted {
        return Err(AppError::NotFound);
    }
    Ok(())
}
