use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use tracing::info;
use sha2::{Sha256, Digest};

use crate::config::Config;
use crate::errors::AppError;
use crate::auth::models::{RegisterRequest, LoginRequest, AuthResponse, RefreshRequest};
use crate::auth::{repository, jwt};
use crate::users::models::UserResponse;

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

pub async fn register(pool: &PgPool, config: &Config, req: RegisterRequest) -> Result<AuthResponse, AppError> {
    // Check if email exists
    if repository::find_user_by_email(pool, &req.email).await?.is_some() {
        return Err(AppError::Conflict("Email already in use".into()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| AppError::InternalError(e.to_string()))?
        .to_string();

    let user_id = Uuid::now_v7();
    let public_id = Uuid::new_v4();

    let mut tx = pool.begin().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

    repository::create_user(&mut *tx, user_id, public_id, &req.display_name)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    repository::create_credential(&mut *tx, user_id, &req.email, &password_hash)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Create tokens
    let access_token = jwt::create_access_token(&public_id, config)?;
    let refresh_token = jwt::create_refresh_token(&public_id, config)?;
    let token_hash = hash_token(&refresh_token);
    let expires_at = Utc::now() + chrono::Duration::seconds(config.jwt_refresh_token_expires_secs as i64);

    repository::create_session(&mut *tx, user_id, &token_hash, expires_at)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

    info!(user_public_id = %public_id, "Created new user");

    Ok(AuthResponse {
        access_token,
        refresh_token,
        user: UserResponse {
            public_id,
            display_name: req.display_name,
            created_at: Utc::now(),
        },
    })
}

pub async fn login(pool: &PgPool, config: &Config, req: LoginRequest) -> Result<AuthResponse, AppError> {
    let (user, cred) = repository::find_user_by_email(pool, &req.email)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".into()))?;

    let parsed_hash = PasswordHash::new(&cred.password_hash)
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    let argon2 = Argon2::default();
    argon2.verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized("Invalid credentials".into()))?;

    let access_token = jwt::create_access_token(&user.public_id, config)?;
    let refresh_token = jwt::create_refresh_token(&user.public_id, config)?;
    let token_hash = hash_token(&refresh_token);
    let expires_at = Utc::now() + chrono::Duration::seconds(config.jwt_refresh_token_expires_secs as i64);

    repository::create_session(pool, user.id, &token_hash, expires_at)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
        user: UserResponse {
            public_id: user.public_id,
            display_name: user.display_name,
            created_at: user.created_at,
        },
    })
}

pub async fn refresh(pool: &PgPool, config: &Config, req: RefreshRequest) -> Result<AuthResponse, AppError> {
    let token_hash = hash_token(&req.refresh_token);

    let session = repository::find_session_by_token_hash(pool, &token_hash)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::Unauthorized("Invalid refresh token".into()))?;

    if session.revoked || session.expires_at < Utc::now() {
        return Err(AppError::Unauthorized("Invalid or expired refresh token".into()));
    }

    let user = repository::find_user_by_id(pool, session.user_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::Unauthorized("User not found".into()))?;

    let mut tx = pool.begin().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

    repository::revoke_session(&mut *tx, session.id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let access_token = jwt::create_access_token(&user.public_id, config)?;
    let refresh_token = jwt::create_refresh_token(&user.public_id, config)?;
    let new_token_hash = hash_token(&refresh_token);
    let expires_at = Utc::now() + chrono::Duration::seconds(config.jwt_refresh_token_expires_secs as i64);

    repository::create_session(&mut *tx, user.id, &new_token_hash, expires_at)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
        user: UserResponse {
            public_id: user.public_id,
            display_name: user.display_name,
            created_at: user.created_at,
        },
    })
}

pub async fn logout(pool: &PgPool, token_hash: &str) -> Result<(), AppError> {
    if let Some(session) = repository::find_session_by_token_hash(pool, token_hash)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
    {
        repository::revoke_session(pool, session.id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    }
    Ok(())
}
