use crate::database;
use crate::database::auth::{AuthorizationDatabase, AuthorizationOk, AuthorizationOutcome};

use super::objects::{LoginError, LoginOk};

pub async fn login<'a>(
    login: &'a str,
    password: &'a str,
    db: database::Conn,
) -> Result<LoginOk, LoginError> {
    match db.login(login, password).await {
        AuthorizationOutcome::Ok(res) => Ok(map_auth_ok(res).await),
        AuthorizationOutcome::NotFound => Err(LoginError::NotFound),
        AuthorizationOutcome::Other => Err(LoginError::Other),
        AuthorizationOutcome::InvalidPassword => Err(LoginError::InvalidPassword),
    }
}

async fn map_auth_ok<'a>(result: AuthorizationOk) -> LoginOk {
    LoginOk {
        uuid: (result.uuid.to_owned()),
        username: (result.username.to_owned()),
        access_token: (result.access_token.to_owned()),
        refresh_token: (result.refresh_token.to_owned()),
    }
}
