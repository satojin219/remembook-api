-- Enable RLS on all tables
ALTER TABLE books ENABLE ROW LEVEL SECURITY;
ALTER TABLE memos ENABLE ROW LEVEL SECURITY;
ALTER TABLE questions ENABLE ROW LEVEL SECURITY;
ALTER TABLE user_answers ENABLE ROW LEVEL SECURITY;
ALTER TABLE purchase_histories ENABLE ROW LEVEL SECURITY;

-- Books policies
CREATE POLICY "Users can create their own books" 
ON books FOR INSERT 
TO authenticated 
WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can view their own books" 
ON books FOR SELECT 
TO authenticated 
USING (auth.uid() = user_id);

CREATE POLICY "Users can update their own books" 
ON books FOR UPDATE 
TO authenticated 
USING (auth.uid() = user_id)
WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can delete their own books" 
ON books FOR DELETE 
TO authenticated 
USING (auth.uid() = user_id);

-- Memos policies
CREATE POLICY "Users can create their own memos" 
ON memos FOR INSERT 
TO authenticated 
WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can view their own memos" 
ON memos FOR SELECT 
TO authenticated 
USING (auth.uid() = user_id);

CREATE POLICY "Users can update their own memos" 
ON memos FOR UPDATE 
TO authenticated 
USING (auth.uid() = user_id)
WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can delete their own memos" 
ON memos FOR DELETE 
TO authenticated 
USING (auth.uid() = user_id);

-- Questions policies
CREATE POLICY "Users can create their own questions" 
ON questions FOR INSERT 
TO authenticated 
WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can view their own questions" 
ON questions FOR SELECT 
TO authenticated 
USING (auth.uid() = user_id);

CREATE POLICY "Users can update their own questions" 
ON questions FOR UPDATE 
TO authenticated 
USING (auth.uid() = user_id)
WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can delete their own questions" 
ON questions FOR DELETE 
TO authenticated 
USING (auth.uid() = user_id);

-- User answers policies
CREATE POLICY "Users can create their own answers" 
ON user_answers FOR INSERT 
TO authenticated 
WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can view their own answers" 
ON user_answers FOR SELECT 
TO authenticated 
USING (auth.uid() = user_id);

CREATE POLICY "Users can update their own answers" 
ON user_answers FOR UPDATE 
TO authenticated 
USING (auth.uid() = user_id)
WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can delete their own answers" 
ON user_answers FOR DELETE 
TO authenticated 
USING (auth.uid() = user_id);

-- Purchase histories policies
CREATE POLICY "Users can create their own purchase histories" 
ON purchase_histories FOR INSERT 
TO authenticated 
WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can view their own purchase histories" 
ON purchase_histories FOR SELECT 
TO authenticated 
USING (auth.uid() = user_id);

-- Coins policies
ALTER TABLE coins ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can create their own coins" 
ON coins FOR INSERT 
TO authenticated 
WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can view their own coins" 
ON coins FOR SELECT 
TO authenticated 
USING (auth.uid() = user_id);

CREATE POLICY "Users can update their own coins" 
ON coins FOR UPDATE 
TO authenticated 
USING (auth.uid() = user_id)
WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can delete their own coins" 
ON coins FOR DELETE 
TO authenticated 
USING (auth.uid() = user_id);