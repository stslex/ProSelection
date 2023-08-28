// @generated automatically by Diesel CLI.

use rocket_contrib::databases::diesel;

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        secret -> Text,
    }
}
