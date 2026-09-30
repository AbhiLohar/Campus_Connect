use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, Executor, FromRow, Postgres, Row};
use uuid::Uuid;

#[derive(Debug)]
pub struct PostRow {
    pub id: Uuid,
    pub public_id: Uuid,
    pub community_id: Uuid,
    pub author_user_id: Uuid,
    pub posting_identity_type: String,
    pub anonymous_identity_id: Option<Uuid>,
    pub post_type: String,
    pub title: String,
    pub body: Option<String>,
    pub link_url: Option<String>,
    pub vote_score: i32,
    pub upvote_count: i32,
    pub downvote_count: i32,
    pub comment_count: i32,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub is_removed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}

impl<'r> FromRow<'r, PgRow> for PostRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            public_id: row.try_get("public_id")?,
            community_id: row.try_get("community_id")?,
            author_user_id: row.try_get("author_user_id")?,
            posting_identity_type: row.try_get("posting_identity_type")?,
            anonymous_identity_id: row.try_get("anonymous_identity_id")?,
            post_type: row.try_get("post_type")?,
            title: row.try_get("title")?,
            body: row.try_get("body")?,
            link_url: row.try_get("link_url")?,
            vote_score: row.try_get("vote_score")?,
            upvote_count: row.try_get("upvote_count")?,
            downvote_count: row.try_get("downvote_count")?,
            comment_count: row.try_get("comment_count")?,
            is_pinned: row.try_get("is_pinned")?,
            is_locked: row.try_get("is_locked")?,
            is_removed: row.try_get("is_removed")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            edited_at: row.try_get("edited_at")?,
        })
    }
}

pub async fn create_post<'e, E>(
    executor: E,
    community_id: Uuid,
    author_user_id: Uuid,
    posting_identity_type: &str,
    anonymous_identity_id: Option<Uuid>,
    post_type: &str,
    title: &str,
    body: Option<&str>,
    link_url: Option<&str>,
) -> Result<PostRow, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let sql = r#"
        INSERT INTO posts (community_id, author_user_id, posting_identity_type, anonymous_identity_id, post_type, title, body, link_url)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
    "#;
    sqlx::query(sql)
        .bind(community_id)
        .bind(author_user_id)
        .bind(posting_identity_type)
        .bind(anonymous_identity_id)
        .bind(post_type)
        .bind(title)
        .bind(body)
        .bind(link_url)
        .try_map(|row: PgRow| PostRow::from_row(&row))
        .fetch_one(executor)
        .await
}

pub async fn find_post_by_public_id(
    pool: &sqlx::PgPool,
    public_id: Uuid,
) -> Result<Option<PostRow>, sqlx::Error> {
    let sql = "SELECT * FROM posts WHERE public_id = $1 AND is_removed = FALSE";
    sqlx::query(sql)
        .bind(public_id)
        .try_map(|row: PgRow| PostRow::from_row(&row))
        .fetch_optional(pool)
        .await
}

pub async fn list_posts_by_community(
    pool: &sqlx::PgPool,
    community_id: Uuid,
    sort: Option<&str>,
    limit: i64,
    before_created_at: Option<DateTime<Utc>>,
) -> Result<Vec<PostRow>, sqlx::Error> {
    let mut sql = String::from("SELECT * FROM posts WHERE community_id = $1 AND is_removed = FALSE");
    if before_created_at.is_some() {
        sql.push_str(" AND created_at < $2");
    }
    
    match sort.unwrap_or("new") {
        "top" => sql.push_str(" ORDER BY vote_score DESC, created_at DESC"),
        "hot" => sql.push_str(" ORDER BY (vote_score + comment_count) DESC, created_at DESC"),
        _ => sql.push_str(" ORDER BY created_at DESC"),
    }
    
    sql.push_str(&format!(" LIMIT {}", limit));
    
    let mut q = sqlx::query(&sql).bind(community_id);
    if let Some(date) = before_created_at {
        q = q.bind(date);
    }
    
    q.try_map(|row: PgRow| PostRow::from_row(&row))
        .fetch_all(pool)
        .await
}

pub async fn list_posts_by_user(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<PostRow>, sqlx::Error> {
    let sql = "SELECT * FROM posts WHERE author_user_id = $1 AND is_removed = FALSE ORDER BY created_at DESC LIMIT $2 OFFSET $3";
    sqlx::query(sql)
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .try_map(|row: PgRow| PostRow::from_row(&row))
        .fetch_all(pool)
        .await
}

pub async fn update_post(
    pool: &sqlx::PgPool,
    post_id: Uuid,
    title: &str,
    body: Option<&str>,
) -> Result<(), sqlx::Error> {
    let sql = "UPDATE posts SET title = $1, body = $2, edited_at = NOW() WHERE id = $3";
    sqlx::query(sql)
        .bind(title)
        .bind(body)
        .bind(post_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_post(pool: &sqlx::PgPool, post_id: Uuid) -> Result<(), sqlx::Error> {
    let sql = "UPDATE posts SET is_removed = TRUE WHERE id = $1";
    sqlx::query(sql).bind(post_id).execute(pool).await?;
    Ok(())
}

pub async fn update_post_vote_counts(pool: &sqlx::PgPool, post_id: Uuid) -> Result<(), sqlx::Error> {
    let sql = r#"
        UPDATE posts 
        SET upvote_count = (SELECT COUNT(*) FROM votes WHERE target_id = $1 AND target_type = 'post' AND value = 1),
            downvote_count = (SELECT COUNT(*) FROM votes WHERE target_id = $1 AND target_type = 'post' AND value = -1),
            vote_score = (SELECT COALESCE(SUM(value), 0) FROM votes WHERE target_id = $1 AND target_type = 'post')
        WHERE id = $1
    "#;
    sqlx::query(sql).bind(post_id).execute(pool).await?;
    Ok(())
}

pub async fn update_post_comment_count(pool: &sqlx::PgPool, post_id: Uuid) -> Result<(), sqlx::Error> {
    let sql = "UPDATE posts SET comment_count = (SELECT COUNT(*) FROM comments WHERE post_id = $1 AND is_removed = FALSE) WHERE id = $1";
    sqlx::query(sql).bind(post_id).execute(pool).await?;
    Ok(())
}
