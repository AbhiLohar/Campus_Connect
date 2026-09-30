use sqlx::{PgPool, Row, postgres::PgRow, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::users::models::UpdateProfileRequest;

#[derive(Debug)]
pub struct UserProfileRow {
    pub public_id: Uuid,
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for UserProfileRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            public_id: row.try_get("public_id")?,
            display_name: row.try_get("display_name")?,
            bio: row.try_get("bio")?,
            avatar_url: row.try_get("avatar_url")?,
            status: row.try_get::<Option<String>, _>("status")?.unwrap_or_else(|| "active".to_string()),
            created_at: row.try_get("created_at")?,
        })
    }
}

pub async fn find_user_profile(pool: &PgPool, user_id: Uuid) -> Result<Option<UserProfileRow>, sqlx::Error> {
    sqlx::query_as::<_, UserProfileRow>(
        "SELECT public_id, display_name, bio, avatar_url, status, created_at FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_user_profile(pool: &PgPool, user_id: Uuid, updates: UpdateProfileRequest) -> Result<Option<UserProfileRow>, sqlx::Error> {
    let current = match find_user_profile(pool, user_id).await? {
        Some(p) => p,
        None => return Ok(None),
    };

    let display_name = updates.display_name.unwrap_or(current.display_name);
    let bio = if updates.bio.is_some() { updates.bio } else { current.bio };
    let avatar_url = if updates.avatar_url.is_some() { updates.avatar_url } else { current.avatar_url };

    sqlx::query_as::<_, UserProfileRow>(
        r#"
        UPDATE users
        SET display_name = $1, bio = $2, avatar_url = $3, updated_at = NOW()
        WHERE id = $4
        RETURNING public_id, display_name, bio, avatar_url, status, created_at
        "#,
    )
    .bind(&display_name)
    .bind(&bio)
    .bind(&avatar_url)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}
