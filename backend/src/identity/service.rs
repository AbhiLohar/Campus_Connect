use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use super::models::PostingIdentity;
use super::repository::{self, AnonymousIdentityRow};
use crate::users::repository as user_repo;

pub async fn get_or_create_identity(
    pool: &PgPool,
    user_id: Uuid,
    community_id: Uuid,
) -> Result<AnonymousIdentityRow, AppError> {
    repository::find_or_create_anonymous_identity(pool, user_id, community_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub async fn get_posting_identity(
    pool: &PgPool,
    user_id: Uuid,
    anonymous_identity_id: Option<Uuid>,
    is_anonymous: bool,
) -> Result<PostingIdentity, AppError> {
    if is_anonymous {
        let anon_id = anonymous_identity_id
            .ok_or_else(|| AppError::BadRequest("Anonymous identity ID required".to_string()))?;
            
        let anon = repository::find_anonymous_identity_by_id(pool, anon_id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Anonymous identity not found".to_string()))?;
            
        if anon.user_id != user_id {
            return Err(AppError::Forbidden("Not your anonymous identity".to_string()));
        }
        
        Ok(PostingIdentity::Anonymous {
            anonymous_id: anon.id,
            display_alias: anon.display_alias,
        })
    } else {
        let user = user_repo::find_user_profile(pool, user_id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
            
        Ok(PostingIdentity::Public {
            user_public_id: user.public_id,
            display_name: user.display_name,
        })
    }
}
