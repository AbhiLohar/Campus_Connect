use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use validator::Validate;

use crate::{
    app::AppState,
    auth::AuthUser,
    errors::AppError,
};
use super::{
    models::{ListingResponse, CreateListingRequest, ListingListQuery},
    service,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/listings", get(list_listings).post(create_listing))
}

async fn list_listings(
    State(state): State<AppState>,
    Query(query): Query<ListingListQuery>,
) -> Result<Json<Vec<ListingResponse>>, AppError> {
    let listings = service::list_listings(&state.db, query).await?;
    Ok(Json(listings))
}

async fn create_listing(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateListingRequest>,
) -> Result<Json<ListingResponse>, AppError> {
    req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let listing = service::create_listing(&state.db, auth_user.user_id, req).await?;
    Ok(Json(listing))
}
