-- Your SQL goes here
CREATE TABLE IF NOT EXISTS favourite
(
    uuid uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT table_favourite_pk PRIMARY KEY,
    user_uuid uuid NOT NULL,
    favourite_uuid uuid NOT NULL,
    title varchar(128) NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS favourite_uuid_uindex ON favourite (uuid);
CREATE INDEX IF NOT EXISTS favourite_user_uuid_uindex ON favourite (user_uuid);
CREATE INDEX IF NOT EXISTS favourite_favorite_uuid_uindex ON favourite (favourite_uuid);
CREATE INDEX IF NOT EXISTS favourite_title_uindex ON favourite (title);
