pub use diesel::ExpressionMethods;
use rocket_contrib::databases::diesel::Insertable;

use crate::schema::users;

mod auth_database;

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub login: &'a str,
    pub username: &'a str,
    pub secret: &'a str,
}

pub trait AuthorizationDatabase {
    fn login(&self, login: &str, password: &str) -> AuthorizationOutcome;
    fn registration(&self, login: &str, username: &str, password: &str) -> RegistrationOutcome;
    fn verify_token(&self, uuid: &str, username: &str) -> VerifyTokenOutcome;
}

pub enum RegistrationOutcome {
    Ok(AuthorizationOk),
    AlreadyInUse,
    WeakPassword,
    Other,
}

pub enum AuthorizationOutcome {
    Ok(AuthorizationOk),
    NotFound,
    Other,
}

pub enum VerifyTokenOutcome {
    Ok(AuthorizationOk),
    NotFound,
    Other,
}

pub struct AuthorizationOk {
    pub uuid: String,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
}
