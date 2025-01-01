-- まず embedding_comparisons は user_answers を参照
DROP TABLE IF EXISTS embedding_comparisons;

-- user_answers は questions と users を参照
DROP TABLE IF EXISTS user_answers;

-- questions は summaries を参照
DROP TABLE IF EXISTS questions;

-- summaries は books と users を参照
DROP TABLE IF EXISTS summaries;

-- books は users を参照
DROP TABLE IF EXISTS books;

-- 最後に誰からも参照されていない users を削除
DROP TABLE IF EXISTS users;

-- 最後にトリガー関数を削除
DROP FUNCTION IF EXISTS set_updated_at ();