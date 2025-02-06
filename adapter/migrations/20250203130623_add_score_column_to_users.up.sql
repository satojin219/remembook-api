-- Add up migration script here
ALTER TABLE users ADD COLUMN score INTEGER DEFAULT 0;
