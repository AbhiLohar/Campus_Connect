use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    auth::AuthUser,
    errors::AppError,
};
use super::{
    models::{NotificationResponse, NotificationListQuery},
    service,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_notifications))
        .route("/read-all", post(mark_all_read))
        .route("/:id/read", post(mark_read))
        .route("/unread-count", get(unread_count))
}

async fn list_notifications(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<NotificationListQuery>,
) -> Result<Json<Vec<NotificationResponse>>, AppError> {
    let notifs = service::list_notifications(&state.db, auth_user.user_id, query).await?;
    Ok(Json(notifs))
}

async fn mark_read(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    service::mark_read(&state.db, auth_user.user_id, id).await
}

async fn mark_all_read(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<(), AppError> {
    service::mark_all_read(&state.db, auth_user.user_id).await
}

async fn unread_count(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<serde_json::Value>, AppError> {
    let count = service::unread_count(&state.db, auth_user.user_id).await?;
    Ok(Json(serde_json::json!({ "count": count })))
}
