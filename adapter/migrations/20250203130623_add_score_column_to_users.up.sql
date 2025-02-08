-- Add up migration script here
ALTER TABLE users ADD COLUMN coins INTEGER DEFAULT 0;
