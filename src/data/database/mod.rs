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

#[derive(Clone)]
pub enum DatabaseResponse<ERROR> {
    Ok,
    Err(ERROR),
}

trait OpenError<T: Clone> {
    fn open_error(&self) -> T;
}

impl<T: Clone> OpenError<DatabaseResponse<T>> for Result<DatabaseResponse<T>, DatabaseResponse<T>> {
    fn open_error(&self) -> DatabaseResponse<T> {
        match self {
            Ok(value) => value.clone(),
            Err(value) => value.clone(),
        }
    }
}
