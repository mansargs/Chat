-- Add migration script here
CREATE TABLE refresh_token (
    refresh_token_id UUID PRIMARY KEY DEFAULT get_random_uuid(),
    user_id UUID NOT NULL,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPZ NOT NULL,
    created_at TIMESTAMPZ NOT NULL DEFAULT now(),
    revoked_at TIMESTAMPZ,

    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);