-- Remove coins column from users
ALTER TABLE users DROP COLUMN coins;

-- Create coins table
CREATE TABLE IF NOT EXISTS coins (
    coin_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    purchased_amount INTEGER NOT NULL,
    used_amount INTEGER NOT NULL DEFAULT 0,
    type VARCHAR(10) NOT NULL CHECK (type IN ('PAID', 'FREE')),
    created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE INDEX idx_coins_user ON coins(user_id, type, created_at);

-- updated_at用のトリガー
CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON coins
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();