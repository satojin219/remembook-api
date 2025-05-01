ALTER TABLE coins RENAME COLUMN type TO coin_type;

-- Drop the old index
DROP INDEX IF EXISTS idx_coins_user;

-- Create the new index with the correct column name
CREATE INDEX idx_coins_user ON coins(user_id, created_at); 