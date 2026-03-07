use axum::Router;
use sqlx::PgPool;

// TODO: implement receipt routes (CRUD + photo upload)
pub fn router() -> Router<PgPool> {
    Router::new()
}
