// @generated automatically by Diesel CLI.

use rocket_contrib::databases::diesel;

diesel::table! {
    users (id) {
        id -> Uuid,
        login -> Varchar,
        username -> Varchar,
        secret -> Text,
        avatar_url -> Varchar,
        bio -> Varchar,
    }
}

diesel::table! {
    follow (follower_uuid, followed_uuid){
        follower_uuid -> Uuid,
        followed_uuid -> Uuid,
        username -> Varchar,
        avatar_url -> Varchar,
    }
}

diesel::table! {
    favourite (uuid, favourite_uuid){
        uuid -> Uuid,
        favourite_uuid -> Uuid,
        title -> Varchar,
    }
}
