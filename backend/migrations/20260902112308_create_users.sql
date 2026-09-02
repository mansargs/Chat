-- Add migration script here
CREATE TABLE users (
    user_id UUID PRIMARY KEY DEFAULT get_random_uuid(),
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPZ NOT NULL DEFAULT now(),
    last_seen_at TIMESTAMPZ, 
);