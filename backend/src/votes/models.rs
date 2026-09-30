use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct VoteRequest {
    pub target_type: String,
    pub target_public_id: Uuid,
    #[validate(range(min = -1, max = 1))]
    pub value: i16,
}
