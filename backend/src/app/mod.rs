use axum::{
    routing::get,
    Router,
    extract::State,
    Json,
};
use std::sync::Arc;
use sqlx::PgPool;
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
    set_header::SetResponseHeaderLayer,
};
use axum::http::{header, HeaderValue};
use serde_json::json;

use crate::config::Config;
use crate::errors::AppError;
use crate::middleware::request_id::request_id_middleware;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: redis::Client,
    pub config: Arc<Config>,
}

pub fn create_router(state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/health", get(health))
        .route("/health/ready", get(ready))
        .nest("/auth", crate::auth::handlers::router())
        .nest("/users", crate::users::handlers::router())
        .nest("/colleges", crate::colleges::handlers::router())
        .nest("/communities", crate::communities::handlers::router())
        .nest("/posts", crate::posts::handlers::router())
        .nest("/comments", crate::comments::handlers::router())
        .nest("/votes", crate::votes::handlers::router())
        .nest("/feed", crate::feed::handlers::router())
        .nest("/chat", crate::chat::handlers::router())
        .nest("/events", crate::events::handlers::router())
        .nest("/marketplace", crate::marketplace::handlers::router())
        .nest("/notifications", crate::notifications::handlers::router())
        .nest("/search", crate::search::handlers::router());

    Router::new()
        .nest("/api/v1", api_routes)
        .layer(axum::middleware::from_fn(request_id_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("x-xss-protection"),
            HeaderValue::from_static("1; mode=block"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .with_state(state)
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

async fn ready(State(state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {
    // Check database
    crate::db::check_health(&state.db).await.map_err(|e| AppError::InternalError(e.to_string()))?;

    // Check Redis
    let mut conn = state.redis.get_multiplexed_async_connection()
        .await
        .map_err(|e| AppError::InternalError(format!("Redis connection failed: {}", e)))?;
    redis::cmd("PING")
        .query_async::<String>(&mut conn)
        .await
        .map_err(|e| AppError::InternalError(format!("Redis ping failed: {}", e)))?;

    Ok(Json(json!({ "status": "ready" })))
}
