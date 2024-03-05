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
    matches (id) {
        id -> Uuid,
        creator_uuid -> Uuid,
        user_uuid -> Array<Uuid>,
        title -> Varchar,
        url -> Varchar,
        description -> Varchar,
    }
}
