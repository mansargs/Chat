-- Add migration script here
CREATE TABLE messages (
    message_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content TEXT NOT NULL,
    sender_id UUID,
    conversation_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    FOREIGN KEY (sender_id) REFERENCES users(user_id) ON DELETE SET NULL,
    FOREIGN KEY (conversation_id) REFERENCES conversations(conversation_id) ON DELETE CASCADE
);

CREATE INDEX idx_messages_conversation_created
ON messages (conversation_id, created_at);