-- Add migration script here
CREATE TABLE conversations (
    conversation_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type TEXT NOT NULL CHECK(type IN ('direct', 'group')),
    name TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()

    CHECK (
        (type = 'group' AND name IS NOT NULL)
        OR
        (type = 'direct')
    )
);