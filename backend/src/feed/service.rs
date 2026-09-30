use crate::{
    errors::AppError,
    posts::models::{PaginatedResponse, PostResponse},
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn home_feed(
    _pool: &PgPool,
    _user_id: Uuid,
    _cursor: Option<String>,
    _limit: Option<i64>,
) -> Result<PaginatedResponse<PostResponse>, AppError> {
    // Dummy implementation
    Ok(PaginatedResponse {
        items: vec![],
        next_cursor: None,
        has_more: false,
    })
}
