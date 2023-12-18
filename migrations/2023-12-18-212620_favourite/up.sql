-- Your SQL goes here
create table IF NOT EXISTS favourite
(
    uuid  uuid default uuid_generate_v4() not null
        constraint table_user_pk
        primary key, 
    user_uuid uuid not null,   
    favourite_uuid uuid not null,
    title varchar(128)                       not null
);

create unique index IF NOT EXISTS favourite_uuid_uindex
    on favourite (uuid);

create index IF NOT EXISTS favourite_user_uuid_uindex
    on favourite (user_uuid);

create index IF NOT EXISTS favourite_favorite_uuid_uindex
    on favourite (favourite_uuid);

create index IF NOT EXISTS favourite_title_uindex
    on favourite (title);

DO $$BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'table_favourite_user'
    ) THEN
        ALTER TABLE IF EXISTS public.favourite
            ADD CONSTRAINT table_favourite_user FOREIGN KEY (user_uuid)
            REFERENCES public.users (id) MATCH SIMPLE
            ON UPDATE NO ACTION
            ON DELETE NO ACTION
            NOT VALID;
    END IF;
END$$;

CREATE INDEX IF NOT EXISTS fki_table_favourite_user
    ON public.follow(followed_uuid);