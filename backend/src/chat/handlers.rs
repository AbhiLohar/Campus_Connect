use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    app::AppState,
    auth::AuthUser,
    errors::AppError,
};
use super::{
    models::{ConversationResponse, MessageResponse, SendMessageRequest, CreateConversationRequest, MessageListQuery},
    service,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/conversations", get(list_conversations).post(create_conversation))
        .route("/conversations/:public_id/messages", get(list_messages))
        .route("/messages", post(send_message))
}

async fn list_conversations(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<ConversationResponse>>, AppError> {
    let convs = service::list_conversations(&state.db, auth_user.user_id).await?;
    Ok(Json(convs))
}

async fn create_conversation(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateConversationRequest>,
) -> Result<Json<ConversationResponse>, AppError> {
    let conv = service::create_conversation(&state.db, auth_user.user_id, req).await?;
    Ok(Json(conv))
}

async fn list_messages(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(public_id): Path<Uuid>,
    Query(query): Query<MessageListQuery>,
) -> Result<Json<Vec<MessageResponse>>, AppError> {
    let msgs = service::list_messages(
        &state.db,
        auth_user.user_id,
        public_id,
        query.limit.unwrap_or(50),
        query.cursor,
    )
    .await?;
    Ok(Json(msgs))
}

async fn send_message(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<SendMessageRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let msg = service::send_message(&state.db, auth_user.user_id, req).await?;
    Ok(Json(msg))
}
