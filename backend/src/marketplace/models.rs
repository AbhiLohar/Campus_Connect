use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct ListingResponse {
    pub public_id: Uuid,
    pub title: String,
    pub description: String,
    pub category: String,
    pub price_cents: i32,
    pub currency: String,
    pub condition: String,
    pub location_area: Option<String>,
    pub image_urls: Vec<String>,
    pub status: String,
    pub seller_display_name: String,
    pub seller_is_verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateListingRequest {
    #[validate(length(min = 1, max = 300))]
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub price_cents: i32,
    #[serde(default = "default_inr")]
    pub currency: String,
    #[serde(default = "default_good")]
    pub condition: String,
    pub location_area: Option<String>,
    #[serde(default)]
    pub image_urls: Vec<String>,
}

fn default_inr() -> String { "INR".to_string() }
fn default_good() -> String { "good".to_string() }

#[derive(Debug, Deserialize)]
pub struct ListingListQuery {
    pub category: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
