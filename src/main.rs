#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use crate::routes::RoutesInitialized;
use database::Conn;
use rocket::{Build, Rocket};

mod config;
pub mod database;
pub mod handlers;
pub mod routes;
mod schema;
pub mod utils;

#[rocket::launch]
fn launch() -> Rocket<Build> {
    rocket::custom(config::from_env())
        .attach(Conn::fairing())
        .mount_routes()
}
