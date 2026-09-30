use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use validator::Validate;

use crate::{
    app::AppState,
    auth::AuthUser,
    errors::AppError,
};
use super::{
    models::{
        CommunityResponse, CommunityDetailResponse, CreateCommunityRequest,
        MemberResponse, CommunityListQuery
    },
    service,
};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_communities).post(create_community))
        .route("/me", get(list_my_communities))
        .route("/:slug", get(get_community))
        .route("/:slug/join", post(join_community))
        .route("/:slug/leave", post(leave_community))
        .route("/:slug/members", get(list_members))
}

async fn list_communities(
    State(state): State<AppState>,
    Query(query): Query<CommunityListQuery>,
) -> Result<Json<Vec<CommunityResponse>>, AppError> {
    let coms = service::list_communities(&state.db, query).await?;
    Ok(Json(coms))
}

async fn create_community(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateCommunityRequest>,
) -> Result<Json<CommunityResponse>, AppError> {
    req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let com = service::create_community(&state.db, auth_user.user_id, req).await?;
    Ok(Json(com))
}

async fn get_community(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<CommunityDetailResponse>, AppError> {
    let com = service::get_community(&state.db, &slug, None).await?;
    Ok(Json(com))
}

async fn join_community(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    auth_user: AuthUser,
) -> Result<Json<()>, AppError> {
    // Expected to join by public id, but let's parse from string just in case,
    // though the request says public_id. If slug is passed, this is fine because we can extract Uuid.
    let pid = Uuid::parse_str(&slug).map_err(|_| AppError::BadRequest("Invalid ID".to_string()))?;
    service::join_community(&state.db, pid, auth_user.user_id).await?;
    Ok(Json(()))
}

async fn leave_community(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    auth_user: AuthUser,
) -> Result<Json<()>, AppError> {
    let pid = Uuid::parse_str(&slug).map_err(|_| AppError::BadRequest("Invalid ID".to_string()))?;
    service::leave_community(&state.db, pid, auth_user.user_id).await?;
    Ok(Json(()))
}

async fn list_members(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<MemberResponse>>, AppError> {
    let members = service::list_members(&state.db, &slug, 50, 0).await?;
    Ok(Json(members))
}

async fn list_my_communities(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<CommunityResponse>>, AppError> {
    let coms = service::list_my_communities(&state.db, auth_user.user_id).await?;
    Ok(Json(coms))
}
