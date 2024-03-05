-- Your SQL goes here
create table IF NOT EXISTS users
(
    id uuid default uuid_generate_v4() not null
        constraint table_name_pk
        primary key,
    login varchar(123) not null,
    username varchar(128) not null,
    secret text not null,
    bio text  not null,
    avatar_url text not null,
);

alter table users add column if not exists bio text not null;
alter table users add column if not exists avatar_url text not null;
ALTER TABLE users DROP COLUMN matches;

create unique index IF NOT EXISTS users_id_uindex on users (id);
create unique index IF NOT EXISTS users_username_uindex on users (username);
create unique index IF NOT EXISTS users_login_uindex on users (login);
create index IF NOT EXISTS users_secret_uindex on users (secret);
create index IF NOT EXISTS users_bio_uindex on users (bio);
create index IF NOT EXISTS users_avatar_url_uindex on users (avatar_url);
