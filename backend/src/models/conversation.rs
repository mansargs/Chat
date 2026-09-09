use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum ConversationType {
    Direct,
    Group,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum MemberRole {
    Admin,
    Member,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Conversation {
    pub conversation_id: Uuid,
    #[sqlx(rename = "type")]
    pub r#type: ConversationType,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConversationMember {
    pub conversation_id: Uuid,
    pub user_id: Uuid,
    pub role: MemberRole,
    pub joined_at: DateTime<Utc>,
    pub hidden_at: Option<DateTime<Utc>>,
}
