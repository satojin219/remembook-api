-- Make session_id nullable
ALTER TABLE coins
ALTER COLUMN session_id
DROP NOT NULL;

-- Drop unique constraint
ALTER TABLE coins
DROP CONSTRAINT IF EXISTS coins_session_id_key;
