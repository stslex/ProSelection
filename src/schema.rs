// @generated automatically by Diesel CLI.

use rocket_contrib::databases::diesel;

diesel::table! {
    users (id) {
        id -> Uuid,
        login -> Varchar,
        username -> Varchar,
        secret -> Text,
    }
}
