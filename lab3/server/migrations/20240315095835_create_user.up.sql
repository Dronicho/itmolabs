-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY NOT NULL,
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  password TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);