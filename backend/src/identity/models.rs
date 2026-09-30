use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnonymousIdentityResponse {
    pub id: Uuid,
    pub display_alias: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PostingIdentity {
    Public {
        user_public_id: Uuid,
        display_name: String,
    },
    Anonymous {
        anonymous_id: Uuid,
        display_alias: String,
    },
}
