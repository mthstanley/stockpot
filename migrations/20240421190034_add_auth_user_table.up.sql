-- Add up migration script here
CREATE TABLE auth_user
  (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    app_user integer references app_user(id)
  );
