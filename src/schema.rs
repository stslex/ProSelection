// @generated automatically by Diesel CLI.

diesel::table! {
    favourite (uuid) {
        uuid -> Uuid,
        user_uuid -> Uuid,
        favourite_uuid -> Uuid,
        title -> Varchar,
    }
}

diesel::table! {
    follow (uuid) {
        uuid -> Uuid,
        follower_uuid -> Uuid,
        followed_uuid -> Uuid,
        followed_username -> Varchar,
        follower_username -> Varchar,
        followed_avatar_url -> Text,
        follower_avatar_url -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        login -> Varchar,
        username -> Varchar,
        secret -> Text,
        bio -> Varchar,
        avatar_url -> Varchar,
    }
}

diesel::table! {
    matches (uuid) {
        uuid -> Uuid,
        creator_uuid -> Uuid,
        participants_uuid -> Array<Uuid>,
        title -> Varchar,
        description -> Varchar,
        cover_url -> Varchar,
        status -> Varchar,
        created_at -> Int8,
        updated_at -> Int8,
        expires_at -> Int8,
    }
}
