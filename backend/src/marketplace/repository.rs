use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

#[derive(Debug)]
pub struct ListingRow {
    pub id: Uuid,
    pub public_id: Uuid,
    pub seller_user_id: Uuid,
    pub title: String,
    pub description: String,
    pub category: String,
    pub price_cents: i32,
    pub currency: String,
    pub condition: String,
    pub location_area: Option<String>,
    pub image_urls: serde_json::Value,
    pub status: String,
    pub college_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for ListingRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            public_id: row.try_get("public_id")?,
            seller_user_id: row.try_get("seller_user_id")?,
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            category: row.try_get("category")?,
            price_cents: row.try_get("price_cents")?,
            currency: row.try_get("currency")?,
            condition: row.try_get("condition")?,
            location_area: row.try_get("location_area")?,
            image_urls: row.try_get("image_urls")?,
            status: row.try_get("status")?,
            college_id: row.try_get("college_id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

pub async fn create_listing(
    pool: &sqlx::PgPool,
    seller_user_id: Uuid,
    title: &str,
    description: &str,
    category: &str,
    price_cents: i32,
    currency: &str,
    condition: &str,
    location_area: Option<&str>,
    image_urls: serde_json::Value,
) -> Result<ListingRow, sqlx::Error> {
    sqlx::query_as::<_, ListingRow>(
        r#"
        INSERT INTO marketplace_listings (seller_user_id, title, description, category, price_cents, currency, condition, location_area, image_urls)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(seller_user_id).bind(title).bind(description).bind(category)
    .bind(price_cents).bind(currency).bind(condition).bind(location_area).bind(image_urls)
    .fetch_one(pool).await
}

pub async fn list_listings(
    pool: &sqlx::PgPool,
    category: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<ListingRow>, sqlx::Error> {
    if let Some(cat) = category {
        sqlx::query_as::<_, ListingRow>(
            "SELECT * FROM marketplace_listings WHERE status = 'active' AND category = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(cat).bind(limit).bind(offset)
        .fetch_all(pool).await
    } else {
        sqlx::query_as::<_, ListingRow>(
            "SELECT * FROM marketplace_listings WHERE status = 'active' ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit).bind(offset)
        .fetch_all(pool).await
    }
}

pub async fn find_listing_by_public_id(
    pool: &sqlx::PgPool,
    public_id: Uuid,
) -> Result<Option<ListingRow>, sqlx::Error> {
    sqlx::query_as::<_, ListingRow>("SELECT * FROM marketplace_listings WHERE public_id = $1")
        .bind(public_id).fetch_optional(pool).await
}
