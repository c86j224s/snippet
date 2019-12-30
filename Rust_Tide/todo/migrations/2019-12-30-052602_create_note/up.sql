-- Your SQL goes here
CREATE TABLE note (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT false,
  created DATETIME NOT NULL,
  updated DATETIME NOT NULL
);
