-- Your SQL goes here
DROP TABLE IF EXISTS matches;

CREATE TABLE IF NOT EXISTS matches
(
    uuid UUID DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT table_matches_pk PRIMARY KEY,
    creator_uuid UUID NOT NULL,
    participants_uuid UUID[] NOT NULL,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    cover_url VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    expires_at BIGINT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS matches_uuid_uindex ON matches (uuid);
CREATE INDEX IF NOT EXISTS matches_creator_uuid_uindex ON matches (creator_uuid);
CREATE INDEX IF NOT EXISTS matches_participants_uuid_uindex ON matches (participants_uuid);
CREATE INDEX IF NOT EXISTS matches_title_uindex ON matches (title);
CREATE INDEX IF NOT EXISTS matches_description_uindex ON matches (description);
CREATE INDEX IF NOT EXISTS matches_cover_url_uindex ON matches (cover_url);
CREATE INDEX IF NOT EXISTS matches_status_uindex ON matches (status);
CREATE INDEX IF NOT EXISTS matches_created_at_uindex ON matches (created_at);
CREATE INDEX IF NOT EXISTS matches_updated_at_uindex ON matches (updated_at);
CREATE INDEX IF NOT EXISTS matches_expires_at_uindex ON matches (expires_at);
