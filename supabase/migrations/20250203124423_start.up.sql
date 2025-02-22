-- Add up migration script here
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS $$
  BEGIN
    NEW.updated_at := now();
    RETURN NEW;
  END;
$$ LANGUAGE plpgsql;

-- ユーザー情報
CREATE TABLE IF NOT EXISTS users (
  user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL UNIQUE,
  password_hash VARCHAR(255) NOT NULL,
  coins INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  logined_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

-- ユーザーが登録した書籍情報
CREATE TABLE IF NOT EXISTS books (
  book_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  title VARCHAR(255) NOT NULL,
  author TEXT[] NOT NULL,
  image_url TEXT NOT NULL,
  google_books_id VARCHAR(255) NOT NULL,
  user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);
CREATE INDEX idx_books ON books(user_id, created_at);

-- ユーザーが要約した情報（memosに変更）
CREATE TABLE IF NOT EXISTS memos (
  memo_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  memo_text TEXT NOT NULL,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  book_id UUID NOT NULL REFERENCES books(book_id) ON DELETE CASCADE
);
CREATE INDEX idx_memos_user_book ON memos(user_id, book_id);

-- ユーザーが書いた要約（memos）から生成された質問
CREATE TABLE IF NOT EXISTS questions (
  question_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  question_text TEXT NOT NULL,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  book_id UUID NOT NULL REFERENCES books(book_id) ON DELETE CASCADE,
  memo_id UUID NOT NULL REFERENCES memos(memo_id) ON DELETE CASCADE
);
CREATE INDEX idx_questions_memo_created ON questions(memo_id, created_at);

-- 質問に対する回答とユーザーの書いた要約（memos）と回答を比較した結果
CREATE TABLE IF NOT EXISTS user_answers (
  answer_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  question_id UUID NOT NULL REFERENCES questions(question_id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  answer_text TEXT NOT NULL,
  score INTEGER CHECK (score >= 0 AND score <= 100),
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);
CREATE INDEX idx_user_answers_question ON user_answers(user_id, question_id);

CREATE TABLE IF NOT EXISTS purchase_histories (
  purchase_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  session_id VARCHAR(255) NOT NULL UNIQUE,
  amount INTEGER NOT NULL,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);
