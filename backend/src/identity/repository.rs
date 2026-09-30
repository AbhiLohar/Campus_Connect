use sqlx::{postgres::PgRow, Executor, FromRow, Postgres, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct AnonymousIdentityRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub community_id: Option<Uuid>,
    pub display_alias: String,
    pub created_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for AnonymousIdentityRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            community_id: row.try_get("community_id")?,
            display_alias: row.try_get("display_alias")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

pub async fn find_or_create_anonymous_identity<'e, E>(
    executor: E,
    user_id: Uuid,
    community_id: Uuid,
) -> Result<AnonymousIdentityRow, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let alias = {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        format!("Anon{}", rng.gen_range(1000..9999))
    };

    sqlx::query_as::<_, AnonymousIdentityRow>(
        r#"
        INSERT INTO anonymous_identities (id, user_id, community_id, display_alias)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (user_id, community_id) DO UPDATE 
        SET display_alias = anonymous_identities.display_alias
        RETURNING *
        "#,
    )
    .bind(uuid::Uuid::now_v7())
    .bind(user_id)
    .bind(community_id)
    .bind(alias)
    .fetch_one(executor)
    .await
}

pub async fn find_anonymous_identity_by_id<'e, E>(
    executor: E,
    id: Uuid,
) -> Result<Option<AnonymousIdentityRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, AnonymousIdentityRow>(
        "SELECT * FROM anonymous_identities WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(executor)
    .await
}
