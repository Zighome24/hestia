pub mod password;
pub mod session;
pub mod webauthn;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use sqlx::PgPool;
use tower_sessions::Session;

use crate::error::AppError;
use crate::models::user::User;

/// Extractor that requires an authenticated user.
/// Use this in route handlers: `async fn handler(auth: AuthUser, ...) -> ...`
pub struct AuthUser(pub User);

#[axum::async_trait]
impl FromRequestParts<PgPool> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, pool: &PgPool) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, pool)
            .await
            .map_err(|_| AppError::Internal)?;

        let user_id = session::get_user_id(&session)
            .await
            .map_err(|_| AppError::Internal)?
            .ok_or(AppError::Unauthorized)?;

        let user = User::find_by_id(pool, user_id)
            .await
            .map_err(|_| AppError::Internal)?
            .ok_or(AppError::Unauthorized)?;

        Ok(AuthUser(user))
    }
}
