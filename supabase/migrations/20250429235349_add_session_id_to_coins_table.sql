-- Add session_id column to coins table
ALTER TABLE coins
ADD COLUMN session_id VARCHAR(255) NOT NULL UNIQUE;

-- Create index for session_id
CREATE INDEX idx_coins_session ON coins (session_id);