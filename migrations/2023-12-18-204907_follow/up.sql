-- Your SQL goes here
CREATE TABLE IF NOT EXISTS follow
(
    uuid UUID DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT table_follow_pk PRIMARY KEY,
    follower_uuid  UUID NOT NULL,
    followed_uuid UUID NOT NULL,
    follower_username VARCHAR(123) NOT NULL,
    followed_username VARCHAR(123) NOT NULL,
    follower_avatar_url TEXT NOT NULL,
    followed_avatar_url TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS follow_uuid_uindex ON follow (uuid);
CREATE INDEX IF NOT EXISTS follow_follower_uuid_uindex ON follow (follower_uuid);
CREATE INDEX IF NOT EXISTS follow_followed_uuid_uindex ON follow (followed_uuid);
CREATE INDEX IF NOT EXISTS follow_follower_username_uindex ON follow (follower_username);
CREATE INDEX IF NOT EXISTS follow_followed_username_uindex ON follow (followed_username);
CREATE INDEX IF NOT EXISTS follow_follower_avatar_url_uindex ON follow (follower_avatar_url);
CREATE INDEX IF NOT EXISTS follow_followed_avatar_url_uindex ON follow (followed_avatar_url);
