use crate::database;
use crate::database::auth::{AuthorizationDatabase, AuthorizationOk, RegistrationOutcome};

use super::objects::LoginOk;

pub type Token = String;

pub enum RegistrationError {
    LoginInUse,
    WeakPassword,
    Other,
}

pub fn registration(
    login: &str,
    password: &str,
    db: database::Conn,
) -> Result<LoginOk, RegistrationError> {
    match db.registration(login, password) {
        RegistrationOutcome::Ok(res) => Ok(map_auth_ok(res)),
        RegistrationOutcome::AlreadyInUse => Err(RegistrationError::LoginInUse),
        RegistrationOutcome::WeakPassword => Err(RegistrationError::WeakPassword),
        _ => Err(RegistrationError::Other),
    }
}

fn map_auth_ok<'a>(result: AuthorizationOk) -> LoginOk {
    LoginOk {
        uuid: (result.uuid.to_owned()),
        username: (result.username.to_owned()),
        token: (result.token.to_owned()),
    }
}
