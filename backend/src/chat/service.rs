use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use super::models::{ConversationResponse, MessageResponse, SendMessageRequest, CreateConversationRequest};
use super::repository;

pub async fn create_conversation(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateConversationRequest,
) -> Result<ConversationResponse, AppError> {
    let mut tx = pool.begin().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let conv = repository::create_conversation(
        &mut *tx,
        &req.conversation_type,
        req.name.as_deref(),
        user_id,
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Add creator as admin
    repository::add_conversation_member(&mut *tx, conv.id, user_id, "public", "admin")
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Add other participants
    for _pid in &req.participant_public_ids {
        // In real implementation, resolve public_id -> user_id
        // For now, skip
    }

    tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(ConversationResponse {
        public_id: conv.public_id,
        conversation_type: conv.conversation_type,
        name: conv.name,
        members: vec![],
        last_message: None,
        updated_at: conv.updated_at,
    })
}

pub async fn list_conversations(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<ConversationResponse>, AppError> {
    let rows = repository::list_user_conversations(pool, user_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|r| ConversationResponse {
            public_id: r.public_id,
            conversation_type: r.conversation_type,
            name: r.name,
            members: vec![],
            last_message: None,
            updated_at: r.updated_at,
        })
        .collect())
}

pub async fn send_message(
    pool: &PgPool,
    user_id: Uuid,
    req: SendMessageRequest,
) -> Result<MessageResponse, AppError> {
    let conv = repository::find_conversation_by_public_id(pool, req.conversation_public_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Conversation not found".to_string()))?;

    // Verify membership
    let is_member = repository::is_conversation_member(pool, conv.id, user_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    if !is_member {
        return Err(AppError::Forbidden("Not a member of this conversation".to_string()));
    }

    let identity_type = if req.is_anonymous { "anonymous" } else { "public" };

    let msg = repository::send_message(
        pool,
        conv.id,
        user_id,
        identity_type,
        None,
        &req.body,
        &req.message_type,
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Update conversation timestamp
    sqlx::query("UPDATE conversations SET updated_at = NOW() WHERE id = $1")
        .bind(conv.id)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(MessageResponse {
        public_id: msg.public_id,
        body: msg.body,
        sender_display_name: "You".to_string(), // Would resolve from user profile
        is_anonymous: req.is_anonymous,
        message_type: msg.message_type,
        media_url: msg.media_url,
        created_at: msg.created_at,
    })
}

pub async fn list_messages(
    pool: &PgPool,
    user_id: Uuid,
    conversation_public_id: Uuid,
    limit: i64,
    cursor: Option<String>,
) -> Result<Vec<MessageResponse>, AppError> {
    let conv = repository::find_conversation_by_public_id(pool, conversation_public_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Conversation not found".to_string()))?;

    let is_member = repository::is_conversation_member(pool, conv.id, user_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    if !is_member {
        return Err(AppError::Forbidden("Not a member".to_string()));
    }

    let before_date = cursor.and_then(|c| chrono::DateTime::parse_from_rfc3339(&c).ok().map(|d| d.with_timezone(&chrono::Utc)));

    let rows = repository::list_messages(pool, conv.id, limit, before_date)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|r| MessageResponse {
            public_id: r.public_id,
            body: r.body,
            sender_display_name: if r.sender_identity_type == "anonymous" {
                "Anonymous".to_string()
            } else {
                "User".to_string()
            },
            is_anonymous: r.sender_identity_type == "anonymous",
            message_type: r.message_type,
            media_url: r.media_url,
            created_at: r.created_at,
        })
        .collect())
}
