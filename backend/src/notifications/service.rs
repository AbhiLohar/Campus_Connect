use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use super::models::{NotificationResponse, NotificationListQuery};
use super::repository;

pub async fn list_notifications(
    pool: &PgPool,
    user_id: Uuid,
    query: NotificationListQuery,
) -> Result<Vec<NotificationResponse>, AppError> {
    let rows = repository::list_notifications(
        pool,
        user_id,
        query.unread_only.unwrap_or(false),
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|r| NotificationResponse {
            id: r.id,
            notification_type: r.notification_type,
            title: r.title,
            body: r.body,
            data: r.data,
            is_read: r.is_read,
            created_at: r.created_at,
        })
        .collect())
}

pub async fn mark_read(pool: &PgPool, user_id: Uuid, notification_id: Uuid) -> Result<(), AppError> {
    repository::mark_notification_read(pool, notification_id, user_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub async fn mark_all_read(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
    repository::mark_all_read(pool, user_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub async fn unread_count(pool: &PgPool, user_id: Uuid) -> Result<i64, AppError> {
    repository::unread_count(pool, user_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
