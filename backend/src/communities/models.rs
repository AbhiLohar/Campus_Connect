use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunityResponse {
    pub public_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub community_type: String,
    pub visibility: String,
    pub join_policy: String,
    pub member_count: i32,
    pub is_official: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleResponse {
    pub rule_number: i32,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunityDetailResponse {
    #[serde(flatten)]
    pub community: CommunityResponse,
    pub rules: Vec<RuleResponse>,
    pub my_role: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommunityRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub visibility: String,
    pub join_policy: String,
    pub college_slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberResponse {
    pub user_public_id: Uuid,
    pub display_name: String,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CommunityListQuery {
    pub college_slug: Option<String>,
    pub community_type: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}
