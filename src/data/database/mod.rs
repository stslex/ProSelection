use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket_sync_db_pools::{database, diesel};

pub mod favourite;
pub mod follow;
pub mod tests;
pub mod user;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[database("diesel_postgres_pool")]
pub struct Conn(diesel::PgConnection);

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
