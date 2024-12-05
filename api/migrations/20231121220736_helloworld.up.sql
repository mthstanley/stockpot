-- Add up migration script here
CREATE TABLE helloworld
  (
     id   SERIAL PRIMARY KEY,
     name VARCHAR(100) NOT NULL
  );
