use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use super::models::{ListingResponse, CreateListingRequest, ListingListQuery};
use super::repository;

pub async fn create_listing(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateListingRequest,
) -> Result<ListingResponse, AppError> {
    let image_urls_json = serde_json::to_value(&req.image_urls)
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    let row = repository::create_listing(
        pool, user_id, &req.title,
        req.description.as_deref().unwrap_or(""),
        &req.category, req.price_cents, &req.currency, &req.condition,
        req.location_area.as_deref(), image_urls_json,
    ).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(to_response(row))
}

pub async fn list_listings(
    pool: &PgPool,
    query: ListingListQuery,
) -> Result<Vec<ListingResponse>, AppError> {
    let rows = repository::list_listings(
        pool,
        query.category.as_deref(),
        query.limit.unwrap_or(20),
        query.offset.unwrap_or(0),
    ).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(rows.into_iter().map(to_response).collect())
}

fn to_response(row: repository::ListingRow) -> ListingResponse {
    let urls: Vec<String> = serde_json::from_value(row.image_urls).unwrap_or_default();
    ListingResponse {
        public_id: row.public_id,
        title: row.title,
        description: row.description,
        category: row.category,
        price_cents: row.price_cents,
        currency: row.currency,
        condition: row.condition,
        location_area: row.location_area,
        image_urls: urls,
        status: row.status,
        seller_display_name: "Seller".to_string(),
        seller_is_verified: false,
        created_at: row.created_at,
    }
}
