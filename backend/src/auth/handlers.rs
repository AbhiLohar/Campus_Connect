use axum::{
    routing::{post, get},
    Router,
    extract::State,
    Json,
};
use validator::Validate;
use sha2::{Sha256, Digest};

use crate::app::AppState;
use crate::errors::AppError;
use crate::auth::models::{RegisterRequest, LoginRequest, AuthResponse, RefreshRequest};
use crate::auth::service;
use crate::auth::middleware::AuthUser;
use crate::users::models::UserResponse;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .route("/me", get(me))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let response = service::register(&state.db, &state.config, payload).await?;
    Ok(Json(response))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let response = service::login(&state.db, &state.config, payload).await?;
    Ok(Json(response))
}

async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = service::refresh(&state.db, &state.config, payload).await?;
    Ok(Json(response))
}

async fn logout(
    State(_state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut hasher = Sha256::new();
    hasher.update(payload.refresh_token.as_bytes());
    let hash = hex::encode(hasher.finalize());
    service::logout(&_state.db, &hash).await?;
    Ok(Json(serde_json::json!({"status": "logged_out"})))
}

async fn me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<UserResponse>, AppError> {
    let user = crate::auth::repository::find_user_by_id(&state.db, auth_user.user_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(Json(UserResponse {
        public_id: user.public_id,
        display_name: user.display_name,
        created_at: user.created_at,
    }))
}
