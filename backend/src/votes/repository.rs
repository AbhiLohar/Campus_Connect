use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

#[derive(Debug)]
pub struct VoteRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub target_type: String,
    pub target_id: Uuid,
    pub value: i16,
}

impl<'r> FromRow<'r, PgRow> for VoteRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            target_type: row.try_get("target_type")?,
            target_id: row.try_get("target_id")?,
            value: row.try_get("value")?,
        })
    }
}

pub async fn upsert_vote(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    target_type: &str,
    target_id: Uuid,
    value: i16,
) -> Result<(), sqlx::Error> {
    let sql = r#"
        INSERT INTO votes (user_id, target_type, target_id, value)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (user_id, target_type, target_id)
        DO UPDATE SET value = $4, updated_at = NOW()
    "#;
    sqlx::query(sql)
        .bind(user_id)
        .bind(target_type)
        .bind(target_id)
        .bind(value)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_vote(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    target_type: &str,
    target_id: Uuid,
) -> Result<(), sqlx::Error> {
    let sql = "DELETE FROM votes WHERE user_id = $1 AND target_type = $2 AND target_id = $3";
    sqlx::query(sql)
        .bind(user_id)
        .bind(target_type)
        .bind(target_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn find_user_vote(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    target_type: &str,
    target_id: Uuid,
) -> Result<Option<VoteRow>, sqlx::Error> {
    let sql = "SELECT * FROM votes WHERE user_id = $1 AND target_type = $2 AND target_id = $3";
    sqlx::query(sql)
        .bind(user_id)
        .bind(target_type)
        .bind(target_id)
        .try_map(|row: PgRow| VoteRow::from_row(&row))
        .fetch_optional(pool)
        .await
}

pub async fn find_user_votes_for_targets(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    target_type: &str,
    target_ids: &[Uuid],
) -> Result<Vec<VoteRow>, sqlx::Error> {
    if target_ids.is_empty() {
        return Ok(vec![]);
    }
    let sql = "SELECT * FROM votes WHERE user_id = $1 AND target_type = $2 AND target_id = ANY($3)";
    sqlx::query(sql)
        .bind(user_id)
        .bind(target_type)
        .bind(target_ids)
        .try_map(|row: PgRow| VoteRow::from_row(&row))
        .fetch_all(pool)
        .await
}
