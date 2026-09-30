use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub public_id: Uuid,
    pub community_slug: String,
    pub title: String,
    pub body: Option<String>,
    pub post_type: String,
    pub link_url: Option<String>,
    pub media_urls: Vec<String>,
    pub posting_identity_type: String,
    pub author: Option<PostAuthor>,
    pub vote_score: i32,
    pub upvote_count: i32,
    pub downvote_count: i32,
    pub comment_count: i32,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub my_vote: Option<i16>,
    pub is_saved: bool,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PostAuthor {
    pub display_name: String,
    pub user_public_id: Option<Uuid>,
    pub is_anonymous: bool,
    pub verified_badge: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostRequest {
    pub community_public_id: Uuid,
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    pub body: Option<String>,
    pub post_type: Option<String>,
    pub link_url: Option<String>,
    #[serde(default)]
    pub is_anonymous: bool,
}

#[derive(Debug, Deserialize)]
pub struct PostListQuery {
    pub sort: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}
