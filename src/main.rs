#![feature(decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use crate::database::TimesheetsDatabaseInitialized;

mod config;
pub mod database;
pub mod routes;

fn main() {
    rocket::custom(config::from_env())
        .manage_database()
        .launch();
}
