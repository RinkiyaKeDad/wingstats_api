-- Add up migration script here
CREATE TABLE IF NOT EXISTS players (
    player_id UUID PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);