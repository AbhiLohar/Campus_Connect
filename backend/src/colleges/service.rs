use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::errors::AppError;
use super::models::{
    CollegeResponse, CollegeDetailResponse, AffiliationResponse,
    CreateAffiliationRequest, GraduateRequest
};
use super::repository;

fn map_college(row: repository::CollegeRow) -> CollegeResponse {
    CollegeResponse {
        public_id: row.public_id,
        name: row.name,
        slug: row.slug,
        short_name: row.short_name,
        city: row.city,
        state: row.state,
        country: row.country,
        logo_url: row.logo_url,
    }
}

pub async fn list_colleges(pool: &PgPool) -> Result<Vec<CollegeResponse>, AppError> {
    let rows = repository::list_colleges(pool).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    Ok(rows.into_iter().map(map_college).collect())
}

pub async fn get_college(pool: &PgPool, slug: &str) -> Result<CollegeDetailResponse, AppError> {
    let row = repository::find_college_by_slug(pool, slug).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("College not found".to_string()))?;
        
    let domains_rows = repository::list_domains_for_college(pool, row.id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    let website = row.website_url.clone();
    let college = map_college(row);
    
    Ok(CollegeDetailResponse {
        college,
        website_url: website,
        domains: domains_rows.into_iter().map(|d| d.domain).collect(),
    })
}

pub async fn create_affiliation(pool: &PgPool, user_id: Uuid, req: CreateAffiliationRequest) -> Result<AffiliationResponse, AppError> {
    let college = repository::find_college_by_public_id(pool, req.college_public_id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("College not found".to_string()))?;
        
    let row = repository::create_affiliation(
        pool,
        user_id,
        college.id,
        req.program,
        req.department,
        req.year_start,
        req.year_end
    ).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    Ok(AffiliationResponse {
        public_id: row.public_id,
        college: map_college(college),
        program: row.program,
        department: row.department,
        year_start: row.year_start,
        year_end: row.year_end,
        status: row.status,
        verification_level: row.verification_level,
    })
}

pub async fn list_my_affiliations(pool: &PgPool, user_id: Uuid) -> Result<Vec<AffiliationResponse>, AppError> {
    let rows = repository::list_user_affiliations(pool, user_id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    let mut responses = Vec::new();
    for row in rows {
        let college_row = sqlx::query_as::<_, repository::CollegeRow>("SELECT * FROM colleges WHERE id = $1")
            .bind(row.college_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            
        responses.push(AffiliationResponse {
            public_id: row.public_id,
            college: map_college(college_row),
            program: row.program,
            department: row.department,
            year_start: row.year_start,
            year_end: row.year_end,
            status: row.status,
            verification_level: row.verification_level,
        });
    }
    
    Ok(responses)
}

pub async fn graduate(pool: &PgPool, user_id: Uuid, req: GraduateRequest) -> Result<AffiliationResponse, AppError> {
    let affiliation = repository::find_affiliation_by_public_id(pool, req.affiliation_public_id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Affiliation not found".to_string()))?;
        
    if affiliation.user_id != user_id {
        return Err(AppError::Forbidden("Not your affiliation".to_string()));
    }
    
    repository::update_affiliation_status(pool, affiliation.id, "alumni", req.year_end).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    let college_row = sqlx::query_as::<_, repository::CollegeRow>("SELECT * FROM colleges WHERE id = $1")
        .bind(affiliation.college_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    Ok(AffiliationResponse {
        public_id: affiliation.public_id,
        college: map_college(college_row),
        program: affiliation.program,
        department: affiliation.department,
        year_start: affiliation.year_start,
        year_end: Some(req.year_end),
        status: "alumni".to_string(),
        verification_level: affiliation.verification_level,
    })
}

pub async fn verify_by_email(pool: &PgPool, user_id: Uuid, email: String) -> Result<AffiliationResponse, AppError> {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(AppError::BadRequest("Invalid email".to_string()));
    }
    let domain = parts[1];
    
    let college = repository::find_college_by_domain(pool, domain).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("College not found for this email domain".to_string()))?;
        
    let affs = repository::list_user_affiliations(pool, user_id).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    let aff = affs.into_iter().find(|a| a.college_id == college.id)
        .ok_or_else(|| AppError::NotFound("No affiliation with this college found".to_string()))?;
        
    repository::update_affiliation_verification(pool, aff.id, 1, Some(Utc::now())).await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
    Ok(AffiliationResponse {
        public_id: aff.public_id,
        college: map_college(college),
        program: aff.program,
        department: aff.department,
        year_start: aff.year_start,
        year_end: aff.year_end,
        status: aff.status,
        verification_level: 1,
    })
}
