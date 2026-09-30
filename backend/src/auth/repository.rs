use sqlx::{PgPool, Row, postgres::PgRow, FromRow, Executor, Postgres};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct UserRow {
    pub id: Uuid,
    pub public_id: Uuid,
    pub display_name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for UserRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            public_id: row.try_get("public_id")?,
            display_name: row.try_get("display_name")?,
            status: row.try_get::<Option<String>, _>("status")?.unwrap_or_else(|| "active".to_string()),
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

#[derive(Debug)]
pub struct CredentialRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub email_verified: bool,
}

impl<'r> FromRow<'r, PgRow> for CredentialRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
            email_verified: row.try_get("email_verified")?,
        })
    }
}

#[derive(Debug)]
pub struct SessionRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub revoked: bool,
}

impl<'r> FromRow<'r, PgRow> for SessionRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            token_hash: row.try_get("token_hash")?,
            expires_at: row.try_get("expires_at")?,
            revoked: row.try_get("revoked")?,
        })
    }
}

pub async fn create_user<'e, E>(executor: E, id: Uuid, public_id: Uuid, display_name: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query(
        "INSERT INTO users (id, public_id, display_name) VALUES ($1, $2, $3)"
    )
    .bind(id)
    .bind(public_id)
    .bind(display_name)
    .execute(executor)
    .await?;
    Ok(())
}

pub async fn create_credential<'e, E>(executor: E, user_id: Uuid, email: &str, password_hash: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query(
        "INSERT INTO user_credentials (user_id, email, password_hash) VALUES ($1, $2, $3)"
    )
    .bind(user_id)
    .bind(email)
    .bind(password_hash)
    .execute(executor)
    .await?;
    Ok(())
}

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Option<(UserRow, CredentialRow)>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT u.id, u.public_id, u.display_name, u.status, u.created_at, u.updated_at,
               c.id as c_id, c.user_id as c_user_id, c.email, c.password_hash, c.email_verified
        FROM users u
        JOIN user_credentials c ON u.id = c.user_id
        WHERE c.email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| {
        let user = UserRow {
            id: r.get("id"),
            public_id: r.get("public_id"),
            display_name: r.get("display_name"),
            status: r.get::<Option<String>, _>("status").unwrap_or_else(|| "active".to_string()),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        };
        let cred = CredentialRow {
            id: r.get("c_id"),
            user_id: r.get("c_user_id"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            email_verified: r.get("email_verified"),
        };
        (user, cred)
    }))
}

pub async fn find_user_by_public_id(pool: &PgPool, public_id: Uuid) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, public_id, display_name, status, created_at, updated_at FROM users WHERE public_id = $1"
    )
    .bind(public_id)
    .fetch_optional(pool)
    .await
}

pub async fn find_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        "SELECT id, public_id, display_name, status, created_at, updated_at FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn find_credential_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Option<CredentialRow>, sqlx::Error> {
    sqlx::query_as::<_, CredentialRow>(
        "SELECT id, user_id, email, password_hash, email_verified FROM user_credentials WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn create_session<'e, E>(executor: E, user_id: Uuid, token_hash: &str, expires_at: DateTime<Utc>) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query(
        "INSERT INTO sessions (user_id, token_hash, expires_at) VALUES ($1, $2, $3)"
    )
    .bind(user_id)
    .bind(token_hash)
    .bind(expires_at)
    .execute(executor)
    .await?;
    Ok(())
}

pub async fn find_session_by_token_hash(pool: &PgPool, token_hash: &str) -> Result<Option<SessionRow>, sqlx::Error> {
    sqlx::query_as::<_, SessionRow>(
        "SELECT id, user_id, token_hash, expires_at, revoked FROM sessions WHERE token_hash = $1"
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await
}

pub async fn revoke_session<'e, E>(executor: E, session_id: Uuid) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query("UPDATE sessions SET revoked = TRUE WHERE id = $1")
        .bind(session_id)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn revoke_all_user_sessions(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE sessions SET revoked = TRUE WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn create_email_verification<'e, E>(executor: E, user_id: Uuid, token: &str, expires_at: DateTime<Utc>) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query(
        "INSERT INTO email_verifications (user_id, token, expires_at) VALUES ($1, $2, $3)"
    )
    .bind(user_id)
    .bind(token)
    .bind(expires_at)
    .execute(executor)
    .await?;
    Ok(())
}

pub async fn verify_email_token(pool: &PgPool, token: &str) -> Result<Option<Uuid>, sqlx::Error> {
    let row = sqlx::query(
        "UPDATE email_verifications SET used = TRUE WHERE token = $1 AND used = FALSE AND expires_at > NOW() RETURNING user_id"
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.get("user_id")))
}
