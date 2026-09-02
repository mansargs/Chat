-- Add migration script here
CREATE TABLE conversation_members (
    conversation_id UUID NOT NULL,
    user_id UUID NOT NULL,
    role TEXT NOT NULL DEFAULT 'member'
        CHECK(role IN ('admin', 'member')),
    joined_at TIMESTAMPZ NOT NULL DEFAULT now(),

    PRIMARY KEY (conversation_id, user_id),

    FOREIGN KEY (conversation_id)
        REFERENCES conversations(conversation_id)
        ON DELETE CASCADE,

    FOREIGN KEY (user_id)
        REFERENCES users(user_id)
        ON DELETE CASCADE
);