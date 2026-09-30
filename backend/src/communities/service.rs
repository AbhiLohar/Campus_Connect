use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use super::models::{
    CommunityResponse, CommunityDetailResponse, RuleResponse,
    CreateCommunityRequest, MemberResponse, CommunityListQuery
};
use super::repository;
use crate::colleges::repository as college_repo;

fn map_community(row: repository::CommunityRow) -> CommunityResponse {
    CommunityResponse {
        public_id: row.public_id,
        name: row.name,
        slug: row.slug,
        description: row.description,
        icon_url: row.icon_url,
        banner_url: row.banner_url,
        community_type: row.community_type,
        visibility: row.visibility,
        join_policy: row.join_policy,
        member_count: row.member_count,
        is_official: row.is_official,
    }
}

pub async fn list_communities(pool: &PgPool, query: CommunityListQuery) -> Result<Vec<CommunityResponse>, AppError> {
    let mut college_id = None;
    if let Some(slug) = query.college_slug {
        let col = college_repo::find_college_by_slug(pool, &slug).await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("College not found".to_string()))?;
        college_id = Some(col.id);
    }
    
    let limit = query.limit.unwrap_or(50);
    let offset = query.page.unwrap_or(0) * limit;
    
    let rows = repository::list_communities(pool, college_id, query.community_type.as_deref(), limit, offset)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    Ok(rows.into_iter().map(map_community).collect())
}

pub async fn get_community(
    pool: &PgPool,
    slug_or_public_id: &str,
    user_id: Option<Uuid>
) -> Result<CommunityDetailResponse, AppError> {
    let row = if let Ok(pid) = Uuid::parse_str(slug_or_public_id) {
        repository::find_community_by_public_id(pool, pid).await
    } else {
        repository::find_community_by_slug(pool, slug_or_public_id).await
    }
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or_else(|| AppError::NotFound("Community not found".to_string()))?;
    
    let rule_rows = repository::list_rules(pool, row.id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    let rules = rule_rows.into_iter().map(|r| RuleResponse {
        rule_number: r.rule_number,
        title: r.title,
        description: r.description,
    }).collect();
    
    let mut my_role = None;
    if let Some(uid) = user_id {
        if let Ok(Some(member)) = repository::find_member(pool, row.id, uid).await {
            my_role = Some(member.role);
        }
    }
    
    Ok(CommunityDetailResponse {
        community: map_community(row),
        rules,
        my_role,
    })
}

pub async fn create_community(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateCommunityRequest
) -> Result<CommunityResponse, AppError> {
    let slug = req.name.to_lowercase().replace(' ', "-");
    
    let mut college_id = None;
    if let Some(c_slug) = req.college_slug {
        let col = college_repo::find_college_by_slug(pool, &c_slug).await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("College not found".to_string()))?;
        college_id = Some(col.id);
    }
    
    let community_type = if college_id.is_some() { "college" } else { "global" }.to_string();
    
    let row = repository::create_community(
        pool,
        req.name,
        slug,
        req.description,
        req.visibility,
        req.join_policy,
        college_id,
        community_type,
        user_id
    ).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    repository::add_member(pool, row.id, user_id, "owner").await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    repository::update_member_count(pool, row.id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    let final_row = repository::find_community_by_public_id(pool, row.public_id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .unwrap();
        
    Ok(map_community(final_row))
}

pub async fn join_community(
    pool: &PgPool,
    community_public_id: Uuid,
    user_id: Uuid
) -> Result<(), AppError> {
    let com = repository::find_community_by_public_id(pool, community_public_id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Community not found".to_string()))?;
        
    let member = repository::find_member(pool, com.id, user_id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    if member.is_some() {
        return Err(AppError::BadRequest("Already a member".to_string()));
    }
    
    repository::add_member(pool, com.id, user_id, "member").await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    repository::update_member_count(pool, com.id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    Ok(())
}

pub async fn leave_community(
    pool: &PgPool,
    community_public_id: Uuid,
    user_id: Uuid
) -> Result<(), AppError> {
    let com = repository::find_community_by_public_id(pool, community_public_id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Community not found".to_string()))?;
        
    let member = repository::find_member(pool, com.id, user_id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::BadRequest("Not a member".to_string()))?;
        
    if member.role == "owner" {
        return Err(AppError::BadRequest("Owner cannot leave".to_string()));
    }
    
    repository::remove_member(pool, com.id, user_id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    repository::update_member_count(pool, com.id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    Ok(())
}

pub async fn list_members(
    pool: &PgPool,
    community_slug: &str,
    limit: i64,
    offset: i64
) -> Result<Vec<MemberResponse>, AppError> {
    let com = repository::find_community_by_slug(pool, community_slug).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Community not found".to_string()))?;
        
    let rows = repository::list_members(pool, com.id, limit, offset).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    Ok(rows.into_iter().map(|(m, name, pid)| MemberResponse {
        user_public_id: pid,
        display_name: name,
        role: m.role,
        joined_at: m.joined_at,
    }).collect())
}

pub async fn list_my_communities(pool: &PgPool, user_id: Uuid) -> Result<Vec<CommunityResponse>, AppError> {
    let rows = repository::list_user_communities(pool, user_id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    Ok(rows.into_iter().map(map_community).collect())
}
