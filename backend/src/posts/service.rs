use crate::{
    errors::AppError,
    posts::{
        models::{CreatePostRequest, PaginatedResponse, PostAuthor, PostListQuery, PostResponse},
        repository::{self, PostRow},
    },
    communities::repository as community_repo,
    identity::repository as identity_repo,
    users::repository as user_repo,
};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

pub async fn build_post_author(_pool: &PgPool, post_row: &PostRow) -> Result<PostAuthor, AppError> {
    if post_row.posting_identity_type == "anonymous" {
        // For anonymous posts: look up the anonymous identity alias
        // CRITICAL: NEVER expose the real user_id
        let alias = if let Some(anon_id) = post_row.anonymous_identity_id {
            match identity_repo::find_anonymous_identity_by_id(_pool, anon_id)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?
            {
                Some(identity) => identity.display_alias,
                None => "Anonymous Student".to_string(),
            }
        } else {
            "Anonymous Student".to_string()
        };

        Ok(PostAuthor {
            display_name: alias,
            user_public_id: None, // NEVER expose for anonymous
            is_anonymous: true,
            verified_badge: None,
        })
    } else {
        // For public posts: look up the user's profile
        match user_repo::find_user_profile(_pool, post_row.author_user_id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
        {
            Some(user) => Ok(PostAuthor {
                display_name: user.display_name,
                user_public_id: Some(user.public_id),
                is_anonymous: false,
                verified_badge: None,
            }),
            None => Ok(PostAuthor {
                display_name: "Deleted User".to_string(),
                user_public_id: None,
                is_anonymous: false,
                verified_badge: None,
            }),
        }
    }
}

fn build_post_response(
    row: PostRow,
    community_slug: String,
    author: PostAuthor,
    my_vote: Option<i16>,
    is_saved: bool,
) -> PostResponse {
    PostResponse {
        public_id: row.public_id,
        community_slug,
        title: row.title,
        body: row.body,
        post_type: row.post_type,
        link_url: row.link_url,
        media_urls: vec![],
        posting_identity_type: row.posting_identity_type,
        author: Some(author),
        vote_score: row.vote_score,
        upvote_count: row.upvote_count,
        downvote_count: row.downvote_count,
        comment_count: row.comment_count,
        is_pinned: row.is_pinned,
        is_locked: row.is_locked,
        my_vote,
        is_saved,
        created_at: row.created_at,
        edited_at: row.edited_at,
    }
}

pub async fn create_post(
    pool: &PgPool,
    user_id: Uuid,
    req: CreatePostRequest,
) -> Result<PostResponse, AppError> {
    // 1. Find community
    let community = community_repo::find_community_by_public_id(pool, req.community_public_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Community not found".to_string()))?;

    // 2. Check membership
    let member = community_repo::find_member(pool, community.id, user_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    if member.is_none() {
        return Err(AppError::Forbidden("You must join the community to post".to_string()));
    }

    // 3. Handle anonymous identity
    let posting_identity_type = if req.is_anonymous { "anonymous" } else { "public" };
    let anonymous_identity_id = if req.is_anonymous {
        let identity = identity_repo::find_or_create_anonymous_identity(pool, user_id, community.id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Some(identity.id)
    } else {
        None
    };

    let post_type = req.post_type.as_deref().unwrap_or("text");

    // 4. Create post
    let row = repository::create_post(
        pool,
        community.id,
        user_id,
        posting_identity_type,
        anonymous_identity_id,
        post_type,
        &req.title,
        req.body.as_deref(),
        req.link_url.as_deref(),
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let author = build_post_author(pool, &row).await?;

    Ok(build_post_response(row, community.slug, author, None, false))
}

pub async fn get_post(
    pool: &PgPool,
    post_public_id: Uuid,
    _viewer_user_id: Option<Uuid>,
) -> Result<PostResponse, AppError> {
    let row = repository::find_post_by_public_id(pool, post_public_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    let author = build_post_author(pool, &row).await?;

    // Get community slug
    let community = community_repo::find_community_by_id(pool, row.community_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let slug = community.map(|c| c.slug).unwrap_or_else(|| "unknown".to_string());

    Ok(build_post_response(row, slug, author, None, false))
}

pub async fn list_community_posts(
    pool: &PgPool,
    community_slug: &str,
    query: PostListQuery,
    _viewer_user_id: Option<Uuid>,
) -> Result<PaginatedResponse<PostResponse>, AppError> {
    let community = community_repo::find_community_by_slug(pool, community_slug)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Community not found".to_string()))?;

    let limit = query.limit.unwrap_or(20);
    let before_date = query
        .cursor
        .and_then(|c| DateTime::<Utc>::from_str(&c).ok());

    let rows = repository::list_posts_by_community(
        pool,
        community.id,
        query.sort.as_deref(),
        limit + 1,
        before_date,
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let mut has_more = false;
    let mut items = vec![];
    let mut next_cursor = None;

    for (i, row) in rows.into_iter().enumerate() {
        if i as i64 == limit {
            has_more = true;
            next_cursor = Some(row.created_at.to_rfc3339());
            break;
        }

        let author = build_post_author(pool, &row).await?;
        items.push(build_post_response(row, community.slug.clone(), author, None, false));
    }

    Ok(PaginatedResponse {
        items,
        next_cursor,
        has_more,
    })
}

pub async fn update_post(
    pool: &PgPool,
    post_public_id: Uuid,
    user_id: Uuid,
    title: &str,
    body: Option<&str>,
) -> Result<PostResponse, AppError> {
    let row = repository::find_post_by_public_id(pool, post_public_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    if row.author_user_id != user_id {
        return Err(AppError::Unauthorized("Not author".to_string()));
    }

    repository::update_post(pool, row.id, title, body)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    get_post(pool, post_public_id, Some(user_id)).await
}

pub async fn delete_post(
    pool: &PgPool,
    post_public_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let row = repository::find_post_by_public_id(pool, post_public_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    if row.author_user_id != user_id {
        return Err(AppError::Unauthorized("Not author".to_string()));
    }

    repository::delete_post(pool, row.id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}
