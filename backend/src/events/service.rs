use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use super::models::{EventResponse, CreateEventRequest, EventListQuery, RsvpRequest};
use super::repository;

pub async fn create_event(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateEventRequest,
) -> Result<EventResponse, AppError> {
    let row = repository::create_event(
        pool, user_id,
        &req.title,
        req.description.as_deref().unwrap_or(""),
        &req.event_type,
        req.location.as_deref(),
        req.is_online,
        req.online_link.as_deref(),
        req.start_time,
        req.end_time,
        req.capacity,
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(to_response(row, None))
}

pub async fn list_events(
    pool: &PgPool,
    query: EventListQuery,
) -> Result<Vec<EventResponse>, AppError> {
    let rows = repository::list_events(
        pool,
        query.upcoming_only.unwrap_or(true),
        query.limit.unwrap_or(20),
        query.offset.unwrap_or(0),
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(rows.into_iter().map(|r| to_response(r, None)).collect())
}

pub async fn rsvp(
    pool: &PgPool,
    user_id: Uuid,
    event_public_id: Uuid,
    req: RsvpRequest,
) -> Result<(), AppError> {
    let event = repository::find_event_by_public_id(pool, event_public_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Event not found".to_string()))?;

    repository::rsvp(pool, event.id, user_id, &req.status)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

fn to_response(row: repository::EventRow, my_status: Option<String>) -> EventResponse {
    EventResponse {
        public_id: row.public_id,
        title: row.title,
        description: row.description,
        event_type: row.event_type,
        location: row.location,
        is_online: row.is_online,
        online_link: row.online_link,
        start_time: row.start_time,
        end_time: row.end_time,
        capacity: row.capacity,
        attendee_count: row.attendee_count,
        is_cancelled: row.is_cancelled,
        my_status,
        created_at: row.created_at,
    }
}
