#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;

use crate::database::AppDatabaseInitialized;
use crate::routes::RoutesInitialized;

mod config;
pub mod database;
pub mod handlers;
pub mod routes;
mod schema;
pub mod utils;

#[rocket::main]
async fn main() {
    rocket::custom(config::from_env())
        .manage_database()
        .mount_routes()
        .launch();
}
