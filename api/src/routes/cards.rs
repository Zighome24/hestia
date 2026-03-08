use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::card::{Card, CardResponse, CreateCardRequest, UpdateCardRequest};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/", get(list_cards).post(create_card))
        .route("/{id}", get(get_card).put(update_card).delete(delete_card))
}

async fn list_cards(
    State(pool): State<PgPool>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<CardResponse>>, AppError> {
    let cards = Card::find_by_user(&pool, user.id).await?;
    Ok(Json(cards.into_iter().map(Into::into).collect()))
}

async fn create_card(
    State(pool): State<PgPool>,
    AuthUser(user): AuthUser,
    Json(req): Json<CreateCardRequest>,
) -> Result<Json<CardResponse>, AppError> {
    Card::validate_nickname(&req.nickname).map_err(AppError::BadRequest)?;
    Card::validate_last_four(&req.last_four).map_err(AppError::BadRequest)?;

    let card = Card::create(&pool, user.id, &req.nickname, &req.last_four).await?;
    Ok(Json(card.into()))
}

async fn get_card(
    State(pool): State<PgPool>,
    AuthUser(user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<CardResponse>, AppError> {
    let card = Card::find_by_id(&pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    if card.user_id != user.id {
        return Err(AppError::NotFound);
    }

    Ok(Json(card.into()))
}

async fn update_card(
    State(pool): State<PgPool>,
    AuthUser(user): AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCardRequest>,
) -> Result<Json<CardResponse>, AppError> {
    if let Some(ref nickname) = req.nickname {
        Card::validate_nickname(nickname).map_err(AppError::BadRequest)?;
    }
    if let Some(ref last_four) = req.last_four {
        Card::validate_last_four(last_four).map_err(AppError::BadRequest)?;
    }

    let card = Card::update(
        &pool,
        id,
        user.id,
        req.nickname.as_deref(),
        req.last_four.as_deref(),
    )
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(card.into()))
}

async fn delete_card(
    State(pool): State<PgPool>,
    AuthUser(user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    let deleted = Card::delete(&pool, id, user.id).await?;
    if !deleted {
        return Err(AppError::NotFound);
    }
    Ok(())
}
