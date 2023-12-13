pub use diesel::ExpressionMethods;
use rocket_contrib::databases::diesel::Insertable;

use crate::schema::users;

use self::reg_objects::{RegistrationData, RegistrationOutcome};

mod auth_database;
pub mod reg_objects;
mod reg_validation;
mod tests;

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub login: &'a str,
    pub username: &'a str,
    pub secret: &'a str,
}

pub trait AuthorizationDatabase {
    fn login(&self, login: &str, password: &str) -> AuthorizationOutcome;
    fn registration(&self, data: RegistrationData) -> RegistrationOutcome;
    fn verify_token(&self, uuid: &str, username: &str) -> VerifyTokenOutcome;
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
