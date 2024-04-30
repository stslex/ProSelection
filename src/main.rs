#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use std::time::SystemTime;

use crate::presenter::routes::RoutesInitialized;
use config::run_db_migrations;
use presenter::catcher::AppCatcher;
use rocket_sync_db_pools::database;

use rocket::{fairing::AdHoc, Build, Rocket};
mod config;
mod data;
mod presenter;
mod schema;
pub mod utils;

#[rocket::launch]
fn launch() -> Rocket<Build> {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    rocket::custom(config::from_env())
        .attach(Conn::fairing())
        .attach(AdHoc::on_ignite("Database Migrations", run_db_migrations))
        .mount_catcher()
        .mount_routes()
}

#[database("diesel_postgres_pool")]
pub struct Conn(pub diesel::PgConnection);
