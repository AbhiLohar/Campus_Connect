use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Serialize)]
pub struct SearchResults {
    pub posts: Vec<PostSearchResult>,
    pub communities: Vec<CommunitySearchResult>,
    pub users: Vec<UserSearchResult>,
}

#[derive(Debug, Serialize)]
pub struct PostSearchResult {
    pub public_id: Uuid,
    pub title: String,
    pub body_snippet: Option<String>,
    pub community_slug: String,
    pub vote_score: i32,
    pub comment_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CommunitySearchResult {
    pub public_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub member_count: i32,
}

#[derive(Debug, Serialize)]
pub struct UserSearchResult {
    pub public_id: Uuid,
    pub display_name: String,
    pub bio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub scope: Option<String>, // posts, communities, users, all
    pub limit: Option<i64>,
}

pub async fn search(
    pool: &PgPool,
    query: &SearchQuery,
) -> Result<SearchResults, AppError> {
    let limit = query.limit.unwrap_or(10);
    let scope = query.scope.as_deref().unwrap_or("all");
    let search_term = format!("%{}%", query.q);

    let posts = if scope == "all" || scope == "posts" {
        sqlx::query(
            r#"
            SELECT p.public_id, p.title, LEFT(p.body, 200) as body_snippet, c.slug as community_slug,
                   p.vote_score, p.comment_count, p.created_at
            FROM posts p
            JOIN communities c ON p.community_id = c.id
            WHERE p.is_removed = FALSE
              AND (p.title ILIKE $1 OR p.body ILIKE $1)
            ORDER BY p.vote_score DESC, p.created_at DESC
            LIMIT $2
            "#,
        )
        .bind(&search_term)
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .iter()
        .map(|row| PostSearchResult {
            public_id: row.get("public_id"),
            title: row.get("title"),
            body_snippet: row.get("body_snippet"),
            community_slug: row.get("community_slug"),
            vote_score: row.get("vote_score"),
            comment_count: row.get("comment_count"),
            created_at: row.get("created_at"),
        })
        .collect()
    } else {
        vec![]
    };

    let communities = if scope == "all" || scope == "communities" {
        sqlx::query(
            r#"
            SELECT public_id, name, slug, description, member_count
            FROM communities
            WHERE name ILIKE $1 OR description ILIKE $1
            ORDER BY member_count DESC
            LIMIT $2
            "#,
        )
        .bind(&search_term)
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .iter()
        .map(|row| CommunitySearchResult {
            public_id: row.get("public_id"),
            name: row.get("name"),
            slug: row.get("slug"),
            description: row.get("description"),
            member_count: row.get("member_count"),
        })
        .collect()
    } else {
        vec![]
    };

    let users = if scope == "all" || scope == "users" {
        sqlx::query(
            r#"
            SELECT public_id, display_name, bio
            FROM users
            WHERE display_name ILIKE $1 OR bio ILIKE $1
            ORDER BY display_name ASC
            LIMIT $2
            "#,
        )
        .bind(&search_term)
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .iter()
        .map(|row| UserSearchResult {
            public_id: row.get("public_id"),
            display_name: row.get("display_name"),
            bio: row.get("bio"),
        })
        .collect()
    } else {
        vec![]
    };

    Ok(SearchResults { posts, communities, users })
}
