-- Add migration script here
CREATE TABLE clients (
  id TEXT,
  name TEXT,
  password TEXT,
  email TEXT,
  token TEXT,
  PRIMARY KEY (id)
)
