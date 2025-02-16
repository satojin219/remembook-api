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

DROP TABLE IF EXISTS purchase_histories;

-- 最後にトリガー関数を削除
DROP FUNCTION IF EXISTS set_updated_at ();

DROP INDEX IF EXISTS idx_summaries_user_book;

DROP INDEX IF EXISTS idx_questions_memo_created;

DROP INDEX IF EXISTS idx_user_answers_question;
