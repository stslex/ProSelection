use super::objects::LoginOk;
use super::AuthValidation;
use crate::data::database;
use crate::data::database::auth::reg_objects::{RegistrationData, RegistrationOutcome};
use crate::data::database::auth::{AuthorizationDatabase, AuthorizationOk};
use crate::utils::AppHasher;

pub async fn registration(
    login: &str,
    username: &str,
    password: &str,
    db: database::Conn,
) -> Result<LoginOk, RegistrationError> {
    let valid_reg_data = RegistrationData {
        login: login,
        username: username,
        password: password,
    }
    .validate()?;
    let hashed_data = RegistrationData {
        login: &valid_reg_data.login.hash().await,
        username: valid_reg_data.username,
        password: &valid_reg_data.password.hash().await,
    };
    match db.registration(&hashed_data).await {
        RegistrationOutcome::Ok(res) => Ok(map_auth_ok(res)),
        RegistrationOutcome::AlreadyInUse => Err(RegistrationError::LoginInUse),
        RegistrationOutcome::Other(_) => Err(RegistrationError::Other),
    }
}

fn map_auth_ok<'a>(result: AuthorizationOk) -> LoginOk {
    LoginOk {
        uuid: (result.uuid.to_owned()),
        username: (result.username.to_owned()),
        access_token: (result.access_token.to_owned()),
        refresh_token: (result.refresh_token.to_owned()),
    }
}

#[derive(Debug, PartialEq)]
pub enum RegistrationError {
    LoginInUse,
    WeakPassword,
    WeakUsername,
    WeakLogin,
    PasswordTooLong,
    EqualLoginPassword,
    Other,
}

impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RegistrationError::WeakPassword => write!(f, "Weak password"),
            RegistrationError::WeakUsername => write!(f, "Weak username"),
            RegistrationError::WeakLogin => write!(f, "Weak login"),
            RegistrationError::PasswordTooLong => write!(f, "Password too long"),
            RegistrationError::EqualLoginPassword => {
                write!(f, "Equal login and password")
            }
            RegistrationError::LoginInUse => write!(f, "Login in use"),
            RegistrationError::Other => write!(f, "other"),
        }
    }
}
