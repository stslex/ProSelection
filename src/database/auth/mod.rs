pub use diesel::ExpressionMethods;
use rocket_contrib::databases::diesel::Insertable;

use crate::schema::users;

mod auth_database;

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub secret: &'a str,
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
