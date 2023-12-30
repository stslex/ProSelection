#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

use crate::database::AppDatabaseInitialized;
use crate::routes::RoutesInitialized;

mod config;
pub mod database;
pub mod handlers;
pub mod routes;
mod schema;
pub mod utils;

#[rocket::launch]
async fn launch() -> Rocket<Build> {
    rocket::custom(config::from_env())
        .manage_database()
        .await
        .mount_routes()
        .await
}
