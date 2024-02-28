-- Your SQL goes here
CREATE TABLE IF NOT EXISTS matches
(
    id UUID DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT table_matches_pk PRIMARY KEY,
    creator_uuid UUID NOT NULL,
    user_uuid UUID[] NOT NULL,
    title VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    description VARCHAR NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS matches_id_uindex ON matches (id);
CREATE INDEX IF NOT EXISTS matches_creator_uuid_uindex ON matches (creator_uuid);
CREATE INDEX IF NOT EXISTS matches_user_uuid_uindex ON matches (user_uuid);
CREATE INDEX IF NOT EXISTS matches_title_uindex ON matches (title);
CREATE INDEX IF NOT EXISTS matches_url_uindex ON matches (url);
CREATE INDEX IF NOT EXISTS matches_description_uindex ON matches (description);
