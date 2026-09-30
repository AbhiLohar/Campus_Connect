use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct ConversationResponse {
    pub public_id: Uuid,
    pub conversation_type: String,
    pub name: Option<String>,
    pub members: Vec<ConversationMemberResponse>,
    pub last_message: Option<MessageResponse>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ConversationMemberResponse {
    pub display_name: String,
    pub is_anonymous: bool,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub public_id: Uuid,
    pub body: String,
    pub sender_display_name: String,
    pub is_anonymous: bool,
    pub message_type: String,
    pub media_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateConversationRequest {
    pub participant_public_ids: Vec<Uuid>,
    pub name: Option<String>,
    #[serde(default = "default_dm")]
    pub conversation_type: String,
}

fn default_dm() -> String { "dm".to_string() }

#[derive(Debug, Deserialize, Validate)]
pub struct SendMessageRequest {
    pub conversation_public_id: Uuid,
    #[validate(length(min = 1, max = 10000))]
    pub body: String,
    #[serde(default)]
    pub is_anonymous: bool,
    #[serde(default = "default_text")]
    pub message_type: String,
}

fn default_text() -> String { "text".to_string() }

#[derive(Debug, Deserialize)]
pub struct MessageListQuery {
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}
