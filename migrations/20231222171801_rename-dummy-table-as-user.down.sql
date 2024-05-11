-- Add down migration script here
ALTER TABLE app_user RENAME TO helloworld;
ALTER TABLE helloworld RENAME CONSTRAINT app_user_pkey TO helloworld_pkey;
ALTER SEQUENCE appe_user_id_seq RENAME TO helloworld_id_seq;
