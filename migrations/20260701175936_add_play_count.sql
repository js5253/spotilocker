-- Add migration script here
ALTER TABLE tracks ADD COLUMN play_count INTEGER DEFAULT 0;