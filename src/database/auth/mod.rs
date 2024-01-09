pub use diesel::ExpressionMethods;
use rocket_sync_db_pools::diesel::Insertable;

use crate::schema::users;

use self::reg_objects::{RegistrationData, RegistrationOutcome};

mod auth_database;
pub mod reg_objects;
mod reg_validation;
mod tests;

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub login: String,
    pub username: String,
    pub secret: String,
    pub avatar_url: String,
    pub bio: String,
}

#[async_trait]
pub trait AuthorizationDatabase {
    async fn login(&self, login: &str, password: &str) -> AuthorizationOutcome;
    async fn registration(&self, data: RegistrationData) -> RegistrationOutcome;
    async fn verify_token(&self, uuid: &str, username: &str) -> VerifyTokenOutcome;
}

#[derive(Debug)]
pub enum AuthorizationOutcome {
    Ok(AuthorizationOk),
    NotFound,
    InvalidPassword,
    Other,
}

impl std::fmt::Display for VerifyTokenOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyTokenOutcome::Ok(_) => write!(f, "Ok"),
            VerifyTokenOutcome::NotFound => write!(f, "NotFound"),
            VerifyTokenOutcome::Other => write!(f, "Other"),
        }
    }
}

#[derive(Debug)]
pub enum VerifyTokenOutcome {
    Ok(AuthorizationOk),
    NotFound,
    Other,
}

impl std::fmt::Display for AuthorizationOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthorizationOutcome::Ok(_) => write!(f, "Ok"),
            AuthorizationOutcome::NotFound => write!(f, "NotFound"),
            AuthorizationOutcome::Other => write!(f, "Other"),
            AuthorizationOutcome::InvalidPassword => write!(f, "Invalid password"),
        }
    }
}

#[derive(Debug)]
pub struct AuthorizationOk {
    pub uuid: String,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
}

impl std::fmt::Display for AuthorizationOk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ uuid: {}, username: {}, access_token: {}, refresh_token: {} }}",
            self.uuid, self.username, self.access_token, self.refresh_token
        )
    }
}
