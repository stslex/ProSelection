use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::databases::diesel;

#[database("diesel_postgres_pool")]
pub struct Conn(diesel::PgConnection);

pub trait TimesheetsDatabaseInitialized {
    fn manage_database(self) -> Self;
}

embed_migrations!("migrations");

impl TimesheetsDatabaseInitialized for Rocket {
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

pub trait AuthorizationDatabase {
    fn login(&self, login: &str, password: &str) -> AuthorizationOutcome;
    fn registration(&self, login: &str, password: &str) -> RegistrationOutcome;
}

pub enum RegistrationOutcome {
    Ok,
    AlreadyInUse,
    WeakPassword,
    Other,
}

pub enum AuthorizationOutcome {
    Ok(String),
    NotFound,
    Other,
}