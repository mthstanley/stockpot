-- Add up migration script here
ALTER TABLE helloworld RENAME TO app_user;
ALTER TABLE app_user RENAME CONSTRAINT helloworld_pkey TO app_user_pkey;
ALTER SEQUENCE helloworld_id_seq RENAME TO app_user_id_seq;
