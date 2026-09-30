use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

#[derive(Debug)]
pub struct EventRow {
    pub id: Uuid,
    pub public_id: Uuid,
    pub title: String,
    pub description: String,
    pub community_id: Option<Uuid>,
    pub college_id: Option<Uuid>,
    pub organizer_user_id: Option<Uuid>,
    pub event_type: String,
    pub location: Option<String>,
    pub is_online: bool,
    pub online_link: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub capacity: Option<i32>,
    pub attendee_count: i32,
    pub is_cancelled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for EventRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            public_id: row.try_get("public_id")?,
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            community_id: row.try_get("community_id")?,
            college_id: row.try_get("college_id")?,
            organizer_user_id: row.try_get("organizer_user_id")?,
            event_type: row.try_get("event_type")?,
            location: row.try_get("location")?,
            is_online: row.try_get("is_online")?,
            online_link: row.try_get("online_link")?,
            start_time: row.try_get("start_time")?,
            end_time: row.try_get("end_time")?,
            capacity: row.try_get("capacity")?,
            attendee_count: row.try_get("attendee_count")?,
            is_cancelled: row.try_get("is_cancelled")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

pub async fn create_event(
    pool: &sqlx::PgPool,
    organizer_user_id: Uuid,
    title: &str,
    description: &str,
    event_type: &str,
    location: Option<&str>,
    is_online: bool,
    online_link: Option<&str>,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
    capacity: Option<i32>,
) -> Result<EventRow, sqlx::Error> {
    sqlx::query_as::<_, EventRow>(
        r#"
        INSERT INTO events (title, description, organizer_user_id, event_type, location, is_online, online_link, start_time, end_time, capacity)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#,
    )
    .bind(title)
    .bind(description)
    .bind(organizer_user_id)
    .bind(event_type)
    .bind(location)
    .bind(is_online)
    .bind(online_link)
    .bind(start_time)
    .bind(end_time)
    .bind(capacity)
    .fetch_one(pool)
    .await
}

pub async fn list_events(
    pool: &sqlx::PgPool,
    upcoming_only: bool,
    limit: i64,
    offset: i64,
) -> Result<Vec<EventRow>, sqlx::Error> {
    if upcoming_only {
        sqlx::query_as::<_, EventRow>(
            "SELECT * FROM events WHERE is_cancelled = FALSE AND start_time > NOW() ORDER BY start_time ASC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, EventRow>(
            "SELECT * FROM events WHERE is_cancelled = FALSE ORDER BY start_time DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }
}

pub async fn find_event_by_public_id(
    pool: &sqlx::PgPool,
    public_id: Uuid,
) -> Result<Option<EventRow>, sqlx::Error> {
    sqlx::query_as::<_, EventRow>("SELECT * FROM events WHERE public_id = $1")
        .bind(public_id)
        .fetch_optional(pool)
        .await
}

pub async fn rsvp(
    pool: &sqlx::PgPool,
    event_id: Uuid,
    user_id: Uuid,
    status: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO event_attendees (event_id, user_id, status)
        VALUES ($1, $2, $3)
        ON CONFLICT (event_id, user_id) DO UPDATE SET status = $3
        "#,
    )
    .bind(event_id)
    .bind(user_id)
    .bind(status)
    .execute(pool)
    .await?;

    // Update attendee count
    sqlx::query(
        "UPDATE events SET attendee_count = (SELECT COUNT(*) FROM event_attendees WHERE event_id = $1 AND status IN ('going', 'interested')) WHERE id = $1",
    )
    .bind(event_id)
    .execute(pool)
    .await?;
    Ok(())
}
