use axum::{
    routing::get,
    Router,
    extract::State,
    Json,
};

use crate::app::AppState;
use crate::errors::AppError;
use crate::auth::middleware::AuthUser;
use crate::users::models::{FullProfileResponse, UpdateProfileRequest};
use crate::users::service;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_me).patch(update_me))
}

async fn get_me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<FullProfileResponse>, AppError> {
    let profile = service::get_profile(&state.db, auth_user.user_id).await?;
    Ok(Json(profile))
}

async fn update_me(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<FullProfileResponse>, AppError> {
    let profile = service::update_profile(&state.db, auth_user.user_id, payload).await?;
    Ok(Json(profile))
}
