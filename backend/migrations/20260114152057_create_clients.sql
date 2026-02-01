-- Add migration script here
CREATE TABLE clients (
  id TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
  name TEXT UNIQUE,
  password TEXT,
  email TEXT UNIQUE,
  token TEXT
)
