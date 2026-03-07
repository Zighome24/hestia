use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use sqlx::PgPool;
use tower_sessions::Session;

use crate::auth::password;
use crate::auth::session as auth_session;
use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::user::{CreateUserRequest, User, UserResponse};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
}

async fn register(
    State(pool): State<PgPool>,
    session: Session,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    // Validate input
    if req.username.trim().is_empty() {
        return Err(AppError::BadRequest("username is required".to_string()));
    }
    if req.password.len() < 8 {
        return Err(AppError::BadRequest(
            "password must be at least 8 characters".to_string(),
        ));
    }
    if req.display_name.trim().is_empty() {
        return Err(AppError::BadRequest(
            "display_name is required".to_string(),
        ));
    }

    // Check if username already exists
    if User::find_by_username(&pool, &req.username)
        .await?
        .is_some()
    {
        return Err(AppError::BadRequest("username already taken".to_string()));
    }

    let password_hash = password::hash_password(&req.password).map_err(|_| AppError::Internal)?;

    let user = User::create(&pool, &req.username, &req.display_name, &password_hash).await?;

    // Auto-login after registration
    auth_session::set_user_id(&session, user.id)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(user.into()))
}

#[derive(serde::Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

async fn login(
    State(pool): State<PgPool>,
    session: Session,
    Json(req): Json<LoginRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = User::find_by_username(&pool, &req.username)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let valid = password::verify_password(&req.password, &user.password_hash)
        .map_err(|_| AppError::Internal)?;

    if !valid {
        return Err(AppError::Unauthorized);
    }

    auth_session::set_user_id(&session, user.id)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(user.into()))
}

async fn logout(session: Session) -> Result<(), AppError> {
    auth_session::clear_session(&session)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(())
}

async fn me(auth: AuthUser) -> Json<UserResponse> {
    Json(auth.0.into())
}
