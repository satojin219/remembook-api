-- Add down migration script here
-- Drop trigger first
DROP TRIGGER IF EXISTS set_timestamp ON coins;

-- Drop coins table
DROP TABLE IF EXISTS coins;

-- Add coins column back to users
ALTER TABLE users ADD COLUMN coins INTEGER NOT NULL DEFAULT 0;