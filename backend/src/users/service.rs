use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::users::models::{FullProfileResponse, UpdateProfileRequest};
use crate::users::repository;

pub async fn get_profile(pool: &PgPool, user_id: Uuid) -> Result<FullProfileResponse, AppError> {
    let profile = repository::find_user_profile(pool, user_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(FullProfileResponse {
        public_id: profile.public_id,
        display_name: profile.display_name,
        bio: profile.bio.unwrap_or_default(),
        avatar_url: profile.avatar_url,
        status: profile.status,
        created_at: profile.created_at,
    })
}

pub async fn update_profile(pool: &PgPool, user_id: Uuid, request: UpdateProfileRequest) -> Result<FullProfileResponse, AppError> {
    let profile = repository::update_user_profile(pool, user_id, request)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(FullProfileResponse {
        public_id: profile.public_id,
        display_name: profile.display_name,
        bio: profile.bio.unwrap_or_default(),
        avatar_url: profile.avatar_url,
        status: profile.status,
        created_at: profile.created_at,
    })
}
