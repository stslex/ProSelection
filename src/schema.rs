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
        username -> Varchar,
        avatar_url -> Text,
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

// diesel_migrations::embed_migrations!();

// diesel::joinable!(favourite -> users (user_uuid));

// diesel::allow_tables_to_appear_in_same_query!(favourite, follow, users,);
