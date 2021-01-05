-- Your SQL goes here
CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  task TEXT NOT NULL,
  complete BOOLEAN NOT NULL DEFAULT 'f',
  published_at TIMESTAMP NOT NULL
)