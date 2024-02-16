#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use crate::presenter::routes::RoutesInitialized;

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

#[database("diesel_postgres_pool")]
pub struct Conn(pub diesel::PgConnection);

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket_sync_db_pools::database;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

impl Conn {
    pub async fn on_ignite(&self) -> Result<(), ()> {
        self.run(|conn| {
            if let Err(e) = conn.run_pending_migrations(MIGRATIONS) {
                eprint!("Failed to run database migrations: {:?}", e);
                return Err(e);
            }
            return Ok(());
        })
        .await
        .map_err(|_| ())
    }
}
