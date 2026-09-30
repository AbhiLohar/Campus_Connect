use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};

use crate::{
    app::AppState,
    errors::AppError,
};
use super::service::{self, SearchQuery, SearchResults};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(search))
}

async fn search(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResults>, AppError> {
    if query.q.len() < 2 {
        return Err(AppError::BadRequest("Search query must be at least 2 characters".to_string()));
    }
    let results = service::search(&state.db, &query).await?;
    Ok(Json(results))
}
