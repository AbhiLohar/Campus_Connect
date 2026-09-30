use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct CollegeResponse {
    pub public_id: Uuid,
    pub name: String,
    pub slug: String,
    pub short_name: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub logo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollegeDetailResponse {
    #[serde(flatten)]
    pub college: CollegeResponse,
    pub website_url: Option<String>,
    pub domains: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffiliationResponse {
    pub public_id: Uuid,
    pub college: CollegeResponse,
    pub program: Option<String>,
    pub department: Option<String>,
    pub year_start: Option<i32>,
    pub year_end: Option<i32>,
    pub status: String,
    pub verification_level: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAffiliationRequest {
    pub college_public_id: Uuid,
    pub program: Option<String>,
    pub department: Option<String>,
    pub year_start: Option<i32>,
    pub year_end: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GraduateRequest {
    pub affiliation_public_id: Uuid,
    pub year_end: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyByEmailRequest {
    #[validate(email)]
    pub email: String,
}
