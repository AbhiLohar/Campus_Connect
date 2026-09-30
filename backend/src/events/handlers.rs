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
    models::{EventResponse, CreateEventRequest, EventListQuery, RsvpRequest},
    service,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_events).post(create_event))
        .route("/:public_id/rsvp", post(rsvp))
}

async fn list_events(
    State(state): State<AppState>,
    Query(query): Query<EventListQuery>,
) -> Result<Json<Vec<EventResponse>>, AppError> {
    let events = service::list_events(&state.db, query).await?;
    Ok(Json(events))
}

async fn create_event(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateEventRequest>,
) -> Result<Json<EventResponse>, AppError> {
    req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let event = service::create_event(&state.db, auth_user.user_id, req).await?;
    Ok(Json(event))
}

async fn rsvp(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(public_id): Path<Uuid>,
    Json(req): Json<RsvpRequest>,
) -> Result<(), AppError> {
    service::rsvp(&state.db, auth_user.user_id, public_id, req).await
}
