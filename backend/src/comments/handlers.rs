use crate::{
    app::AppState,
    auth::AuthUser,
    comments::{
        models::{CommentResponse, CreateCommentRequest},
        service,
    },
    errors::AppError,
};
use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_comment))
        .route("/post/:post_public_id", get(list_comments))
        .route("/:public_id", delete(delete_comment))
}

async fn create_comment(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateCommentRequest>,
) -> Result<Json<CommentResponse>, AppError> {
    let res = service::create_comment(&state.db, auth_user.user_id, req).await?;
    Ok(Json(res))
}

#[derive(Deserialize)]
pub struct ListCommentsQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn list_comments(
    State(state): State<AppState>,
    Path(post_public_id): Path<Uuid>,
    Query(query): Query<ListCommentsQuery>,
) -> Result<Json<Vec<CommentResponse>>, AppError> {
    let res = service::list_comments(
        &state.db,
        post_public_id,
        None,
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    )
    .await?;
    Ok(Json(res))
}

async fn delete_comment(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(public_id): Path<Uuid>,
) -> Result<(), AppError> {
    service::delete_comment(&state.db, public_id, auth_user.user_id).await?;
    Ok(())
}
