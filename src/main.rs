#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use crate::presenter::routes::RoutesInitialized;
use data::database::Conn;
use rocket::{Build, Rocket};

mod config;
pub mod data;
pub mod presenter;
mod schema;
pub mod utils;

#[rocket::launch]
fn launch() -> Rocket<Build> {
    rocket::custom(config::from_env())
        .attach(Conn::fairing())
        .mount_routes()
}
