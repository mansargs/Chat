use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub message_id: Uuid,
    pub content: String,
    pub sender_id: Option<Uuid>,
    pub conversation_id: Uuid,
    pub created_at: DateTime<Utc>,
}