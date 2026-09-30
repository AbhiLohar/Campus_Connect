use crate::{
    comments::{
        models::{CommentResponse, CreateCommentRequest},
        repository::{self, CommentRow},
    },
    errors::AppError,
    posts::models::PostAuthor,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn build_comment_author(
    _pool: &PgPool,
    row: &CommentRow,
) -> Result<PostAuthor, AppError> {
    if row.posting_identity_type == "anonymous" {
        Ok(PostAuthor {
            display_name: "Anonymous".to_string(),
            user_public_id: None,
            is_anonymous: true,
            verified_badge: None,
        })
    } else {
        Ok(PostAuthor {
            display_name: "Public User".to_string(),
            user_public_id: Some(Uuid::new_v4()),
            is_anonymous: false,
            verified_badge: None,
        })
    }
}

pub async fn create_comment(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateCommentRequest,
) -> Result<CommentResponse, AppError> {
    let post_id = Uuid::new_v4(); // Dummy fetch post logic
    let parent_id = req.parent_comment_public_id; // Dummy logic to fetch internal ID
    let posting_identity_type = if req.is_anonymous { "anonymous" } else { "public" };
    let depth = if parent_id.is_some() { 1 } else { 0 };

    if depth > 10 {
        return Err(AppError::BadRequest("Max depth exceeded".to_string()));
    }

    let row = repository::create_comment(
        pool,
        post_id,
        parent_id,
        user_id,
        posting_identity_type,
        None,
        &req.body,
        depth,
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let author = build_comment_author(pool, &row).await?;

    Ok(CommentResponse {
        public_id: row.public_id,
        post_public_id: req.post_public_id,
        parent_comment_public_id: req.parent_comment_public_id,
        body: row.body,
        author,
        depth: row.depth,
        vote_score: row.vote_score,
        created_at: row.created_at,
        edited_at: row.edited_at,
    })
}

pub async fn list_comments(
    pool: &PgPool,
    post_public_id: Uuid,
    _viewer_user_id: Option<Uuid>,
    limit: i64,
    offset: i64,
) -> Result<Vec<CommentResponse>, AppError> {
    let post_id = Uuid::new_v4(); // Dummy
    let rows = repository::list_comments_by_post(pool, post_id, limit, offset)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let mut res = vec![];
    for row in rows {
        let author = build_comment_author(pool, &row).await?;
        res.push(CommentResponse {
            public_id: row.public_id,
            post_public_id,
            parent_comment_public_id: None,
            body: row.body,
            author,
            depth: row.depth,
            vote_score: row.vote_score,
            created_at: row.created_at,
            edited_at: row.edited_at,
        });
    }
    Ok(res)
}

pub async fn delete_comment(
    pool: &PgPool,
    comment_public_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let row = repository::find_comment_by_public_id(pool, comment_public_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

    if row.author_user_id != user_id {
        return Err(AppError::Unauthorized("Not author".to_string()));
    }

    repository::delete_comment(pool, row.id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}
