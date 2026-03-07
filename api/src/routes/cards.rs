use axum::Router;
use sqlx::PgPool;

// TODO: implement card routes (credit card management)
pub fn router() -> Router<PgPool> {
    Router::new()
}
