use crate::posts::models::PostAuthor;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub public_id: Uuid,
    pub post_public_id: Uuid,
    pub parent_comment_public_id: Option<Uuid>,
    pub body: String,
    pub author: PostAuthor,
    pub depth: i32,
    pub vote_score: i32,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    pub post_public_id: Uuid,
    pub parent_comment_public_id: Option<Uuid>,
    #[validate(length(min = 1, max = 10000))]
    pub body: String,
    #[serde(default)]
    pub is_anonymous: bool,
}
