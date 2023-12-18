-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
create table IF NOT EXISTS users
(
    id       uuid default uuid_generate_v4() not null
        constraint table_name_pk
            primary key,
    login varchar(123)                       not null,   
    username varchar(128)                    not null,
    secret   text                            not null,
    avatar_url varchar(128)                  not null,
    bio varchar(128)                         not null,
);

create unique index IF NOT EXISTS users_id_uindex
    on users (id);

create unique index IF NOT EXISTS users_username_uindex
    on users (username);

create unique index IF NOT EXISTS users_login_uindex
    on users (login);

create unique index IF NOT EXISTS avatar_url_uindex
    on users (avatar_url);

create unique index IF NOT EXISTS bio_uindex
    on users (bio);

create table IF NOT EXISTS follow
(
    id      follower_uuid default not null constraint table_name_pk primary key,
    id      followed_uuid default not null constraint table_name_pk primary key,
    username varchar(128)                    not null,
    avatar_url text                          not null,
);

create table IF NOT EXISTS favourite
(
    id      uuid default not null constraint table_name_pk primary key,
    id      favourite_uuid default not null constraint table_name_pk primary key,
    title varchar(128)                    not null,
)
