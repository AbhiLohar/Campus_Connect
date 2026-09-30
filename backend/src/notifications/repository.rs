use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

#[derive(Debug)]
pub struct NotificationRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: String,
    pub title: String,
    pub body: Option<String>,
    pub data: serde_json::Value,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for NotificationRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            notification_type: row.try_get("notification_type")?,
            title: row.try_get("title")?,
            body: row.try_get("body")?,
            data: row.try_get("data")?,
            is_read: row.try_get("is_read")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

pub async fn create_notification(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    notification_type: &str,
    title: &str,
    body: Option<&str>,
    data: serde_json::Value,
) -> Result<NotificationRow, sqlx::Error> {
    sqlx::query_as::<_, NotificationRow>(
        r#"
        INSERT INTO notifications (user_id, notification_type, title, body, data)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(notification_type)
    .bind(title)
    .bind(body)
    .bind(data)
    .fetch_one(pool)
    .await
}

pub async fn list_notifications(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    unread_only: bool,
    limit: i64,
    offset: i64,
) -> Result<Vec<NotificationRow>, sqlx::Error> {
    if unread_only {
        sqlx::query_as::<_, NotificationRow>(
            "SELECT * FROM notifications WHERE user_id = $1 AND is_read = FALSE ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, NotificationRow>(
            "SELECT * FROM notifications WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }
}

pub async fn mark_notification_read(
    pool: &sqlx::PgPool,
    notification_id: Uuid,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE notifications SET is_read = TRUE WHERE id = $1 AND user_id = $2")
        .bind(notification_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn mark_all_read(
    pool: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE notifications SET is_read = TRUE WHERE user_id = $1 AND is_read = FALSE")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn unread_count(
    pool: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND is_read = FALSE")
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}
