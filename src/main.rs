#![feature(decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use crate::database::AppDatabaseInitialized;
use crate::routes::RoutesInitialized;

mod config;
pub mod database;
pub mod handlers;
pub mod routes;
mod schema;
pub mod utils;

fn main() {
    rocket::custom(config::from_env())
        .manage_database()
        .mount_routes()
        .launch();
}
