use sqlx::{postgres::PgRow, Executor, FromRow, Postgres, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct CollegeRow {
    pub id: Uuid,
    pub public_id: Uuid,
    pub name: String,
    pub slug: String,
    pub short_name: Option<String>,
    pub logo_url: Option<String>,
    pub website_url: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
}

impl<'r> FromRow<'r, PgRow> for CollegeRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            public_id: row.try_get("public_id")?,
            name: row.try_get("name")?,
            slug: row.try_get("slug")?,
            short_name: row.try_get("short_name")?,
            logo_url: row.try_get("logo_url")?,
            website_url: row.try_get("website_url")?,
            city: row.try_get("city")?,
            state: row.try_get("state")?,
            country: row.try_get("country")?,
        })
    }
}

#[derive(Debug)]
pub struct CollegeDomainRow {
    pub id: Uuid,
    pub college_id: Uuid,
    pub domain: String,
    pub is_primary: bool,
}

impl<'r> FromRow<'r, PgRow> for CollegeDomainRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            college_id: row.try_get("college_id")?,
            domain: row.try_get("domain")?,
            is_primary: row.try_get("is_primary")?,
        })
    }
}

#[derive(Debug)]
pub struct AffiliationRow {
    pub id: Uuid,
    pub public_id: Uuid,
    pub user_id: Uuid,
    pub college_id: Uuid,
    pub program: Option<String>,
    pub department: Option<String>,
    pub year_start: Option<i32>,
    pub year_end: Option<i32>,
    pub status: String,
    pub verification_level: i32,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for AffiliationRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            public_id: row.try_get("public_id")?,
            user_id: row.try_get("user_id")?,
            college_id: row.try_get("college_id")?,
            program: row.try_get("program")?,
            department: row.try_get("department")?,
            year_start: row.try_get("year_start")?,
            year_end: row.try_get("year_end")?,
            status: row.try_get("status")?,
            verification_level: row.try_get("verification_level")?,
            verified_at: row.try_get("verified_at")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

pub async fn list_colleges<'e, E>(executor: E) -> Result<Vec<CollegeRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, CollegeRow>("SELECT * FROM colleges ORDER BY name ASC")
        .fetch_all(executor)
        .await
}

pub async fn find_college_by_slug<'e, E>(executor: E, slug: &str) -> Result<Option<CollegeRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, CollegeRow>("SELECT * FROM colleges WHERE slug = $1")
        .bind(slug)
        .fetch_optional(executor)
        .await
}

pub async fn find_college_by_public_id<'e, E>(executor: E, public_id: Uuid) -> Result<Option<CollegeRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, CollegeRow>("SELECT * FROM colleges WHERE public_id = $1")
        .bind(public_id)
        .fetch_optional(executor)
        .await
}

pub async fn find_college_by_domain<'e, E>(executor: E, domain: &str) -> Result<Option<CollegeRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, CollegeRow>(
        "SELECT c.* FROM colleges c JOIN college_domains d ON c.id = d.college_id WHERE d.domain = $1"
    )
        .bind(domain)
        .fetch_optional(executor)
        .await
}

pub async fn list_domains_for_college<'e, E>(executor: E, college_id: Uuid) -> Result<Vec<CollegeDomainRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, CollegeDomainRow>("SELECT * FROM college_domains WHERE college_id = $1")
        .bind(college_id)
        .fetch_all(executor)
        .await
}

pub async fn create_affiliation<'e, E>(
    executor: E,
    user_id: Uuid,
    college_id: Uuid,
    program: Option<String>,
    department: Option<String>,
    year_start: Option<i32>,
    year_end: Option<i32>,
) -> Result<AffiliationRow, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, AffiliationRow>(
        r#"
        INSERT INTO affiliations (public_id, user_id, college_id, program, department, year_start, year_end, status, verification_level)
        VALUES ($1, $2, $3, $4, $5, $6, $7, 'active', 0)
        RETURNING *
        "#
    )
        .bind(uuid::Uuid::now_v7())
        .bind(user_id)
        .bind(college_id)
        .bind(program)
        .bind(department)
        .bind(year_start)
        .bind(year_end)
        .fetch_one(executor)
        .await
}

pub async fn list_user_affiliations<'e, E>(executor: E, user_id: Uuid) -> Result<Vec<AffiliationRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, AffiliationRow>("SELECT * FROM affiliations WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(executor)
        .await
}

pub async fn find_affiliation_by_public_id<'e, E>(executor: E, public_id: Uuid) -> Result<Option<AffiliationRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, AffiliationRow>("SELECT * FROM affiliations WHERE public_id = $1")
        .bind(public_id)
        .fetch_optional(executor)
        .await
}

pub async fn update_affiliation_status<'e, E>(executor: E, affiliation_id: Uuid, status: &str, year_end: i32) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query("UPDATE affiliations SET status = $1, year_end = $2 WHERE id = $3")
        .bind(status)
        .bind(year_end)
        .bind(affiliation_id)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn update_affiliation_verification<'e, E>(executor: E, affiliation_id: Uuid, level: i32, verified_at: Option<DateTime<Utc>>) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query("UPDATE affiliations SET verification_level = $1, verified_at = $2 WHERE id = $3")
        .bind(level)
        .bind(verified_at)
        .bind(affiliation_id)
        .execute(executor)
        .await?;
    Ok(())
}
