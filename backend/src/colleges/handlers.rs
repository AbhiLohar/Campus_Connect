use axum::{
    extract::{Path, State},
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
        CollegeResponse, CollegeDetailResponse, AffiliationResponse,
        CreateAffiliationRequest, GraduateRequest, VerifyByEmailRequest,
    },
    service,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_colleges))
        .route("/:slug", get(get_college_detail))
        .route("/affiliations", post(create_affiliation))
        .route("/affiliations/me", get(list_my_affiliations))
        .route("/affiliations/graduate", post(graduate))
        .route("/affiliations/verify", post(verify_email))
}

async fn list_colleges(
    State(state): State<AppState>,
) -> Result<Json<Vec<CollegeResponse>>, AppError> {
    let colleges = service::list_colleges(&state.db).await?;
    Ok(Json(colleges))
}

async fn get_college_detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<CollegeDetailResponse>, AppError> {
    let college = service::get_college(&state.db, &slug).await?;
    Ok(Json(college))
}

async fn create_affiliation(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateAffiliationRequest>,
) -> Result<Json<AffiliationResponse>, AppError> {
    req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let aff = service::create_affiliation(&state.db, auth_user.user_id, req).await?;
    Ok(Json(aff))
}

async fn list_my_affiliations(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<AffiliationResponse>>, AppError> {
    let affs = service::list_my_affiliations(&state.db, auth_user.user_id).await?;
    Ok(Json(affs))
}

async fn graduate(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<GraduateRequest>,
) -> Result<Json<AffiliationResponse>, AppError> {
    let aff = service::graduate(&state.db, auth_user.user_id, req).await?;
    Ok(Json(aff))
}

async fn verify_email(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<VerifyByEmailRequest>,
) -> Result<Json<AffiliationResponse>, AppError> {
    req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let aff = service::verify_by_email(&state.db, auth_user.user_id, req.email).await?;
    Ok(Json(aff))
}
