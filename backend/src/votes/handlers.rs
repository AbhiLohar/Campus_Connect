use crate::{
    app::AppState,
    auth::AuthUser,
    errors::AppError,
    votes::{models::VoteRequest, service},
};
use axum::{
    extract::{Json, State},
    routing::{delete, post},
    Router,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/vote", post(vote))
        .route("/vote", delete(remove_vote))
}

async fn vote(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<VoteRequest>,
) -> Result<(), AppError> {
    service::vote(&state.db, auth_user.user_id, &req.target_type, req.target_public_id, req.value).await?;
    Ok(())
}

async fn remove_vote(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<VoteRequest>,
) -> Result<(), AppError> {
    service::remove_vote(&state.db, auth_user.user_id, &req.target_type, req.target_public_id).await?;
    Ok(())
}
