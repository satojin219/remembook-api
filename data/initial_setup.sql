-- Insert seed data into the users table
INSERT INTO users (name, email, password_hash)
VALUES
  ('佐藤仁','user1@example.com', '$2b$10$KcA3Q4J8b8JYXf9Roq5zPeFvIeYsAphGJQ2DOxO4gHG3YISZZQ5si'),
  ('佐藤じん','user2@example.com', '$2b$10$A3B4X8C7d9YZXyN5Pq6T7PfFvVeUsUpyHJQ1DOxP4gHG3YTZYQ5he'),
  ('佐藤ジン','user3@example.com', '$2b$10$ZxZbA2V9F7C6YNoPqR8XPfXyYeTsUpHJGJQ5OOxP4gHG4YJZQ6pjq');
