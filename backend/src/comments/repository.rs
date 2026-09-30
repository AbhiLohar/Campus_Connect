use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, Executor, FromRow, Postgres, Row};
use uuid::Uuid;

#[derive(Debug)]
pub struct CommentRow {
    pub id: Uuid,
    pub public_id: Uuid,
    pub post_id: Uuid,
    pub parent_comment_id: Option<Uuid>,
    pub author_user_id: Uuid,
    pub posting_identity_type: String,
    pub anonymous_identity_id: Option<Uuid>,
    pub body: String,
    pub depth: i32,
    pub vote_score: i32,
    pub upvote_count: i32,
    pub downvote_count: i32,
    pub is_removed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}

impl<'r> FromRow<'r, PgRow> for CommentRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            public_id: row.try_get("public_id")?,
            post_id: row.try_get("post_id")?,
            parent_comment_id: row.try_get("parent_comment_id")?,
            author_user_id: row.try_get("author_user_id")?,
            posting_identity_type: row.try_get("posting_identity_type")?,
            anonymous_identity_id: row.try_get("anonymous_identity_id")?,
            body: row.try_get("body")?,
            depth: row.try_get("depth")?,
            vote_score: row.try_get("vote_score")?,
            upvote_count: row.try_get("upvote_count")?,
            downvote_count: row.try_get("downvote_count")?,
            is_removed: row.try_get("is_removed")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            edited_at: row.try_get("edited_at")?,
        })
    }
}

pub async fn create_comment<'e, E>(
    executor: E,
    post_id: Uuid,
    parent_comment_id: Option<Uuid>,
    author_user_id: Uuid,
    posting_identity_type: &str,
    anonymous_identity_id: Option<Uuid>,
    body: &str,
    depth: i32,
) -> Result<CommentRow, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let sql = r#"
        INSERT INTO comments (post_id, parent_comment_id, author_user_id, posting_identity_type, anonymous_identity_id, body, depth)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
    "#;
    sqlx::query(sql)
        .bind(post_id)
        .bind(parent_comment_id)
        .bind(author_user_id)
        .bind(posting_identity_type)
        .bind(anonymous_identity_id)
        .bind(body)
        .bind(depth)
        .try_map(|row: PgRow| CommentRow::from_row(&row))
        .fetch_one(executor)
        .await
}

pub async fn list_comments_by_post(
    pool: &sqlx::PgPool,
    post_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<CommentRow>, sqlx::Error> {
    let sql = "SELECT * FROM comments WHERE post_id = $1 AND is_removed = FALSE ORDER BY created_at ASC LIMIT $2 OFFSET $3";
    sqlx::query(sql)
        .bind(post_id)
        .bind(limit)
        .bind(offset)
        .try_map(|row: PgRow| CommentRow::from_row(&row))
        .fetch_all(pool)
        .await
}

pub async fn find_comment_by_public_id(
    pool: &sqlx::PgPool,
    public_id: Uuid,
) -> Result<Option<CommentRow>, sqlx::Error> {
    let sql = "SELECT * FROM comments WHERE public_id = $1 AND is_removed = FALSE";
    sqlx::query(sql)
        .bind(public_id)
        .try_map(|row: PgRow| CommentRow::from_row(&row))
        .fetch_optional(pool)
        .await
}

pub async fn delete_comment(pool: &sqlx::PgPool, comment_id: Uuid) -> Result<(), sqlx::Error> {
    let sql = "UPDATE comments SET is_removed = TRUE WHERE id = $1";
    sqlx::query(sql).bind(comment_id).execute(pool).await?;
    Ok(())
}

pub async fn update_comment_vote_counts(pool: &sqlx::PgPool, comment_id: Uuid) -> Result<(), sqlx::Error> {
    let sql = r#"
        UPDATE comments 
        SET upvote_count = (SELECT COUNT(*) FROM votes WHERE target_id = $1 AND target_type = 'comment' AND value = 1),
            downvote_count = (SELECT COUNT(*) FROM votes WHERE target_id = $1 AND target_type = 'comment' AND value = -1),
            vote_score = (SELECT COALESCE(SUM(value), 0) FROM votes WHERE target_id = $1 AND target_type = 'comment')
        WHERE id = $1
    "#;
    sqlx::query(sql).bind(comment_id).execute(pool).await?;
    Ok(())
}
