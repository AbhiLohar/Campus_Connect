use sqlx::{postgres::PgRow, Executor, FromRow, Postgres, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct CommunityRow {
    pub id: Uuid,
    pub public_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub college_id: Option<Uuid>,
    pub community_type: String,
    pub visibility: String,
    pub join_policy: String,
    pub member_count: i32,
    pub is_official: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for CommunityRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            public_id: row.try_get("public_id")?,
            name: row.try_get("name")?,
            slug: row.try_get("slug")?,
            description: row.try_get("description")?,
            icon_url: row.try_get("icon_url")?,
            banner_url: row.try_get("banner_url")?,
            college_id: row.try_get("college_id")?,
            community_type: row.try_get("community_type")?,
            visibility: row.try_get("visibility")?,
            join_policy: row.try_get("join_policy")?,
            member_count: row.try_get("member_count")?,
            is_official: row.try_get("is_official")?,
            created_by: row.try_get("created_by")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

#[derive(Debug)]
pub struct CommunityMemberRow {
    pub id: Uuid,
    pub community_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for CommunityMemberRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            community_id: row.try_get("community_id")?,
            user_id: row.try_get("user_id")?,
            role: row.try_get("role")?,
            joined_at: row.try_get("joined_at")?,
        })
    }
}

#[derive(Debug)]
pub struct CommunityRuleRow {
    pub id: Uuid,
    pub community_id: Uuid,
    pub rule_number: i32,
    pub title: String,
    pub description: Option<String>,
}

impl<'r> FromRow<'r, PgRow> for CommunityRuleRow {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            community_id: row.try_get("community_id")?,
            rule_number: row.try_get("rule_number")?,
            title: row.try_get("title")?,
            description: row.try_get("description")?,
        })
    }
}

pub async fn list_communities<'e, E>(
    executor: E,
    college_id: Option<Uuid>,
    community_type: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<CommunityRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let mut query = "SELECT * FROM communities WHERE 1=1".to_string();
    
    if college_id.is_some() {
        query.push_str(" AND college_id = $1");
    }
    if community_type.is_some() {
        if college_id.is_some() {
            query.push_str(" AND community_type = $2");
        } else {
            query.push_str(" AND community_type = $1");
        }
    }
    
    let final_sql = format!("{} ORDER BY member_count DESC LIMIT {} OFFSET {}", query, limit, offset);
    let mut q = sqlx::query_as::<_, CommunityRow>(&final_sql);
    
    if let Some(cid) = college_id {
        q = q.bind(cid);
    }
    if let Some(ct) = community_type {
        q = q.bind(ct);
    }

    q.fetch_all(executor).await
}

pub async fn find_community_by_public_id<'e, E>(
    executor: E,
    public_id: Uuid,
) -> Result<Option<CommunityRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, CommunityRow>("SELECT * FROM communities WHERE public_id = $1")
        .bind(public_id)
        .fetch_optional(executor)
        .await
}

pub async fn find_community_by_slug<'e, E>(
    executor: E,
    slug: &str,
) -> Result<Option<CommunityRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, CommunityRow>("SELECT * FROM communities WHERE slug = $1")
        .bind(slug)
        .fetch_optional(executor)
        .await
}

pub async fn find_community_by_id<'e, E>(
    executor: E,
    id: Uuid,
) -> Result<Option<CommunityRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, CommunityRow>("SELECT * FROM communities WHERE id = $1")
        .bind(id)
        .fetch_optional(executor)
        .await
}

pub async fn create_community<'e, E>(
    executor: E,
    name: String,
    slug: String,
    description: Option<String>,
    visibility: String,
    join_policy: String,
    college_id: Option<Uuid>,
    community_type: String,
    created_by: Uuid,
) -> Result<CommunityRow, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, CommunityRow>(
        r#"
        INSERT INTO communities (public_id, name, slug, description, visibility, join_policy, college_id, community_type, created_by, member_count)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 0)
        RETURNING *
        "#
    )
        .bind(uuid::Uuid::now_v7())
        .bind(name)
        .bind(slug)
        .bind(description)
        .bind(visibility)
        .bind(join_policy)
        .bind(college_id)
        .bind(community_type)
        .bind(created_by)
        .fetch_one(executor)
        .await
}

pub async fn update_member_count<'e, E>(executor: E, community_id: Uuid) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query("UPDATE communities SET member_count = (SELECT COUNT(*) FROM community_members WHERE community_id = $1) WHERE id = $1")
        .bind(community_id)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn add_member<'e, E>(
    executor: E,
    community_id: Uuid,
    user_id: Uuid,
    role: &str,
) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query(
        "INSERT INTO community_members (community_id, user_id, role) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
    )
        .bind(community_id)
        .bind(user_id)
        .bind(role)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn remove_member<'e, E>(
    executor: E,
    community_id: Uuid,
    user_id: Uuid,
) -> Result<(), sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query("DELETE FROM community_members WHERE community_id = $1 AND user_id = $2")
        .bind(community_id)
        .bind(user_id)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn find_member<'e, E>(
    executor: E,
    community_id: Uuid,
    user_id: Uuid,
) -> Result<Option<CommunityMemberRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, CommunityMemberRow>(
        "SELECT * FROM community_members WHERE community_id = $1 AND user_id = $2"
    )
        .bind(community_id)
        .bind(user_id)
        .fetch_optional(executor)
        .await
}

pub async fn list_members<'e, E>(
    executor: E,
    community_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<(CommunityMemberRow, String, Uuid)>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    let rows = sqlx::query(
        r#"
        SELECT m.*, u.display_name, u.public_id as user_public_id
        FROM community_members m
        JOIN users u ON m.user_id = u.id
        WHERE m.community_id = $1
        ORDER BY m.joined_at DESC
        LIMIT $2 OFFSET $3
        "#
    )
        .bind(community_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(executor)
        .await?;

    let mut result = Vec::new();
    for row in rows {
        let member = CommunityMemberRow {
            id: row.try_get("id")?,
            community_id: row.try_get("community_id")?,
            user_id: row.try_get("user_id")?,
            role: row.try_get("role")?,
            joined_at: row.try_get("joined_at")?,
        };
        let display_name: String = row.try_get("display_name")?;
        let public_id: Uuid = row.try_get("user_public_id")?;
        result.push((member, display_name, public_id));
    }
    Ok(result)
}

pub async fn list_rules<'e, E>(
    executor: E,
    community_id: Uuid,
) -> Result<Vec<CommunityRuleRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, CommunityRuleRow>("SELECT * FROM community_rules WHERE community_id = $1 ORDER BY rule_number ASC")
        .bind(community_id)
        .fetch_all(executor)
        .await
}

pub async fn list_user_communities<'e, E>(
    executor: E,
    user_id: Uuid,
) -> Result<Vec<CommunityRow>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, CommunityRow>(
        r#"
        SELECT c.*
        FROM communities c
        JOIN community_members m ON c.id = m.community_id
        WHERE m.user_id = $1
        ORDER BY c.name ASC
        "#
    )
        .bind(user_id)
        .fetch_all(executor)
        .await
}
