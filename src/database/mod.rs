use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::databases::diesel;

pub mod auth;
pub mod tests;
pub mod user;

#[database("diesel_postgres_pool")]
pub struct Conn(diesel::PgConnection);

pub trait AppDatabaseInitialized {
    fn manage_database(self) -> Self;
}

impl AppDatabaseInitialized for Rocket {
    fn manage_database(self) -> Self {
        self.attach(Conn::fairing())
            .attach(AdHoc::on_attach("Running migration", |r| {
                if let Some(c) = Conn::get_one(&r) {
                    if let Err(e) = embedded_migrations::run(&*c) {
                        eprint!("Failed to run database migrations: {:?}", e);
                        return Err(r);
                    }
                }
                return Ok(r);
            }))
    }
}

embed_migrations!("migrations");
