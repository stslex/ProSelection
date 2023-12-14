use super::objects::LoginOk;
use crate::database;
use crate::database::auth::reg_objects::{
    RegistrationData, RegistrationFieldValidError, RegistrationOutcome,
};
use crate::database::auth::{AuthorizationDatabase, AuthorizationOk};

pub enum RegistrationError {
    LoginInUse,
    WeakPassword,
    WeakUsername,
    WeakLogin,
    PasswordTooLong,
    EqualLoginPassword,
    Other,
}

pub fn registration(
    login: &str,
    username: &str,
    password: &str,
    db: database::Conn,
) -> Result<LoginOk, RegistrationError> {
    match db.registration(RegistrationData {
        login: login,
        username: username,
        password: password,
    }) {
        RegistrationOutcome::Ok(res) => Ok(map_auth_ok(res)),
        RegistrationOutcome::AlreadyInUse => Err(RegistrationError::LoginInUse),
        RegistrationOutcome::RegistrationFieldValid(err) => match err {
            RegistrationFieldValidError::WeakPassword => Err(RegistrationError::WeakPassword),
            RegistrationFieldValidError::WeakUsername => Err(RegistrationError::WeakUsername),
            RegistrationFieldValidError::WeakLogin => Err(RegistrationError::WeakLogin),
            RegistrationFieldValidError::PasswordTooLong => Err(RegistrationError::PasswordTooLong),
            RegistrationFieldValidError::EqualLoginPassword => {
                Err(RegistrationError::EqualLoginPassword)
            }
        },
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
