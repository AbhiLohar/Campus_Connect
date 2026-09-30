use crate::{
    app::AppState,
    auth::AuthUser,
    errors::AppError,
    feed::service,
    posts::models::{PaginatedResponse, PostResponse},
};
use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;

pub fn router() -> Router<AppState> {
    Router::new().route("/home", get(home_feed))
}

#[derive(Deserialize)]
pub struct FeedQuery {
    cursor: Option<String>,
    limit: Option<i64>,
}

async fn home_feed(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<FeedQuery>,
) -> Result<Json<PaginatedResponse<PostResponse>>, AppError> {
    let res = service::home_feed(&state.db, auth_user.user_id, query.cursor, query.limit).await?;
    Ok(Json(res))
}
