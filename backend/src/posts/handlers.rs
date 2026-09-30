use axum::{
    extract::{Path, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    auth::AuthUser,
    errors::AppError,
    posts::{
        models::{CreatePostRequest, PostResponse},
        service,
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_post))
        .route("/:public_id", get(get_post))
        .route("/:public_id", patch(update_post))
        .route("/:public_id", delete(delete_post))
}

async fn create_post(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreatePostRequest>,
) -> Result<Json<PostResponse>, AppError> {
    let res = service::create_post(&state.db, auth_user.user_id, req).await?;
    Ok(Json(res))
}

async fn get_post(
    State(state): State<AppState>,
    Path(public_id): Path<Uuid>,
) -> Result<Json<PostResponse>, AppError> {
    let res = service::get_post(&state.db, public_id, None).await?;
    Ok(Json(res))
}

#[derive(serde::Deserialize)]
pub struct UpdatePostRequest {
    title: String,
    body: Option<String>,
}

async fn update_post(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(public_id): Path<Uuid>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<PostResponse>, AppError> {
    let res = service::update_post(&state.db, public_id, auth_user.user_id, &req.title, req.body.as_deref()).await?;
    Ok(Json(res))
}

async fn delete_post(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(public_id): Path<Uuid>,
) -> Result<(), AppError> {
    service::delete_post(&state.db, public_id, auth_user.user_id).await?;
    Ok(())
}
