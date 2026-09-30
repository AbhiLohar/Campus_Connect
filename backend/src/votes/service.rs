use crate::{
    errors::AppError,
    votes::repository,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn vote(
    pool: &PgPool,
    user_id: Uuid,
    target_type: &str,
    _target_id_public: Uuid,
    value: i16,
) -> Result<(), AppError> {
    let target_id = Uuid::new_v4(); // Dummy
    repository::upsert_vote(pool, user_id, target_type, target_id, value)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(())
}

pub async fn remove_vote(
    pool: &PgPool,
    user_id: Uuid,
    target_type: &str,
    _target_id_public: Uuid,
) -> Result<(), AppError> {
    let target_id = Uuid::new_v4(); // Dummy
    repository::delete_vote(pool, user_id, target_type, target_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(())
}
