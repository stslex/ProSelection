use crate::data::repository::auth::objects::{AuthDataError, AuthDataResponse};
use crate::data::repository::auth::AuthRepository;
use crate::Conn;

use crate::utils::AppHasher;

use super::objects::{LoginError, LoginOk};

pub async fn login<'a>(login: &'a str, password: &'a str, db: Conn) -> Result<LoginOk, LoginError> {
    match db.login(&login.hash().await, &password.hash().await).await {
        Result::Ok(res) => Ok(map_auth_ok(res).await),
        Result::Err(AuthDataError::NotFound) => Err(LoginError::NotFound),
        Result::Err(AuthDataError::Other) => Err(LoginError::Other),
        Result::Err(AuthDataError::InvalidPassword) => Err(LoginError::InvalidPassword),
    }
}

async fn map_auth_ok<'a>(result: AuthDataResponse) -> LoginOk {
    LoginOk {
        uuid: (result.uuid.to_owned()),
        username: (result.username.to_owned()),
        access_token: (result.access_token.to_owned()),
        refresh_token: (result.refresh_token.to_owned()),
    }
}
