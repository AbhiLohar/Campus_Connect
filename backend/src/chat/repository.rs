use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, Executor, FromRow, Postgres, Row};
use uuid::Uuid;

#[derive(Debug)]
pub struct ConversationRow {
    pub id: Uuid,
    pub public_id: Uuid,
    pub conversation_type: String,
    pub name: Option<String>,
    pub community_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for ConversationRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            public_id: row.try_get("public_id")?,
            conversation_type: row.try_get("conversation_type")?,
            name: row.try_get("name")?,
            community_id: row.try_get("community_id")?,
            created_by: row.try_get("created_by")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

#[derive(Debug)]
pub struct MessageRow {
    pub id: Uuid,
    pub public_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_user_id: Uuid,
    pub sender_identity_type: String,
    pub anonymous_identity_id: Option<Uuid>,
    pub body: String,
    pub message_type: String,
    pub media_url: Option<String>,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}

impl<'r> FromRow<'r, PgRow> for MessageRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            public_id: row.try_get("public_id")?,
            conversation_id: row.try_get("conversation_id")?,
            sender_user_id: row.try_get("sender_user_id")?,
            sender_identity_type: row.try_get("sender_identity_type")?,
            anonymous_identity_id: row.try_get("anonymous_identity_id")?,
            body: row.try_get("body")?,
            message_type: row.try_get("message_type")?,
            media_url: row.try_get("media_url")?,
            is_deleted: row.try_get("is_deleted")?,
            created_at: row.try_get("created_at")?,
            edited_at: row.try_get("edited_at")?,
        })
    }
}

pub async fn create_conversation<'e, E>(
    executor: E,
    conversation_type: &str,
    name: Option<&str>,
    created_by: Uuid,
) -> Result<ConversationRow, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, ConversationRow>(
        r#"
        INSERT INTO conversations (conversation_type, name, created_by)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(conversation_type)
    .bind(name)
    .bind(created_by)
    .fetch_one(executor)
    .await
}

pub async fn add_conversation_member<'e, E>(
    executor: E,
    conversation_id: Uuid,
    user_id: Uuid,
    identity_type: &str,
    role: &str,
) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query(
        "INSERT INTO conversation_members (conversation_id, user_id, identity_type, role) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
    )
    .bind(conversation_id)
    .bind(user_id)
    .bind(identity_type)
    .bind(role)
    .execute(executor)
    .await?;
    Ok(())
}

pub async fn list_user_conversations(
    pool: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<Vec<ConversationRow>, sqlx::Error> {
    sqlx::query_as::<_, ConversationRow>(
        r#"
        SELECT c.* FROM conversations c
        JOIN conversation_members cm ON c.id = cm.conversation_id
        WHERE cm.user_id = $1
        ORDER BY c.updated_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn find_conversation_by_public_id(
    pool: &sqlx::PgPool,
    public_id: Uuid,
) -> Result<Option<ConversationRow>, sqlx::Error> {
    sqlx::query_as::<_, ConversationRow>(
        "SELECT * FROM conversations WHERE public_id = $1",
    )
    .bind(public_id)
    .fetch_optional(pool)
    .await
}

pub async fn send_message<'e, E>(
    executor: E,
    conversation_id: Uuid,
    sender_user_id: Uuid,
    sender_identity_type: &str,
    anonymous_identity_id: Option<Uuid>,
    body: &str,
    message_type: &str,
) -> Result<MessageRow, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, MessageRow>(
        r#"
        INSERT INTO messages (conversation_id, sender_user_id, sender_identity_type, anonymous_identity_id, body, message_type)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(conversation_id)
    .bind(sender_user_id)
    .bind(sender_identity_type)
    .bind(anonymous_identity_id)
    .bind(body)
    .bind(message_type)
    .fetch_one(executor)
    .await
}

pub async fn list_messages(
    pool: &sqlx::PgPool,
    conversation_id: Uuid,
    limit: i64,
    before_created_at: Option<DateTime<Utc>>,
) -> Result<Vec<MessageRow>, sqlx::Error> {
    if let Some(before) = before_created_at {
        sqlx::query_as::<_, MessageRow>(
            "SELECT * FROM messages WHERE conversation_id = $1 AND is_deleted = FALSE AND created_at < $2 ORDER BY created_at DESC LIMIT $3",
        )
        .bind(conversation_id)
        .bind(before)
        .bind(limit)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, MessageRow>(
            "SELECT * FROM messages WHERE conversation_id = $1 AND is_deleted = FALSE ORDER BY created_at DESC LIMIT $2",
        )
        .bind(conversation_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }
}

pub async fn is_conversation_member(
    pool: &sqlx::PgPool,
    conversation_id: Uuid,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query("SELECT 1 FROM conversation_members WHERE conversation_id = $1 AND user_id = $2")
        .bind(conversation_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    Ok(row.is_some())
}
