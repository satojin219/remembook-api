-- Coins policies
ALTER TABLE coins ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can create their own coins" ON coins FOR INSERT TO authenticated
WITH
  CHECK (auth.uid () = user_id);

CREATE POLICY "Users can view their own coins" ON coins FOR
SELECT
  TO authenticated USING (auth.uid () = user_id);

CREATE POLICY "Users can update their own coins" ON coins FOR
UPDATE TO authenticated USING (auth.uid () = user_id)
WITH
  CHECK (auth.uid () = user_id);

CREATE POLICY "Users can delete their own coins" ON coins FOR DELETE TO authenticated USING (auth.uid () = user_id);