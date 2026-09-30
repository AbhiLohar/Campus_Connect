use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct EventResponse {
    pub public_id: Uuid,
    pub title: String,
    pub description: String,
    pub event_type: String,
    pub location: Option<String>,
    pub is_online: bool,
    pub online_link: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub capacity: Option<i32>,
    pub attendee_count: i32,
    pub is_cancelled: bool,
    pub my_status: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateEventRequest {
    #[validate(length(min = 1, max = 300))]
    pub title: String,
    pub description: Option<String>,
    pub community_public_id: Option<Uuid>,
    #[serde(default = "default_general")]
    pub event_type: String,
    pub location: Option<String>,
    #[serde(default)]
    pub is_online: bool,
    pub online_link: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub capacity: Option<i32>,
}

fn default_general() -> String { "general".to_string() }

#[derive(Debug, Deserialize)]
pub struct EventListQuery {
    pub college_slug: Option<String>,
    pub event_type: Option<String>,
    pub upcoming_only: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct RsvpRequest {
    pub status: String, // going, interested, not_going
}
