-- Your SQL goes here
CREATE TABLE IF NOT EXISTS FOLLOW
(
    uuid UUID DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT table_follow_pk PRIMARY KEY,
    follower_uuid  UUID NOT NULL,
    followed_uuid UUID NOT NULL,
    follower_username VARCHAR(123),
    followed_username VARCHAR(123),
    follower_avatar_url TEXT,
    followed_avatar_url TEXT
);

ALTER TABLE FOLLOW ADD COLUMN IF NOT EXISTS follower_username VARCHAR(123);
ALTER TABLE FOLLOW ADD COLUMN IF NOT EXISTS followed_username VARCHAR(123);
ALTER TABLE FOLLOW ADD COLUMN IF NOT EXISTS follower_avatar_url TEXT;
ALTER TABLE FOLLOW ADD COLUMN IF NOT EXISTS followed_avatar_url TEXT;

ALTER TABLE FOLLOW DROP COLUMN if exists username;
ALTER TABLE FOLLOW DROP COLUMN if exists avatar_url;

CREATE UNIQUE INDEX IF NOT EXISTS follow_uuid_uindex ON FOLLOW (uuid);
CREATE INDEX IF NOT EXISTS follow_follower_uuid_uindex ON FOLLOW (follower_uuid);
CREATE INDEX IF NOT EXISTS follow_followed_uuid_uindex ON FOLLOW (followed_uuid);
CREATE INDEX IF NOT EXISTS follow_follower_username_uindex ON FOLLOW (follower_username);
CREATE INDEX IF NOT EXISTS follow_followed_username_uindex ON FOLLOW (followed_username);
CREATE INDEX IF NOT EXISTS follow_follower_avatar_url_uindex ON FOLLOW (follower_avatar_url);
CREATE INDEX IF NOT EXISTS follow_followed_avatar_url_uindex ON FOLLOW (followed_avatar_url);
