use crate::data::{
    database::{self},
    repository::auth::{objects::VerifyTokenError, AuthRepository},
};
use serde::Serialize;

pub async fn refresh(
    uuid: &str,
    username: &str,
    db: database::Conn,
) -> Result<RefreshOk, RefreshError> {
    db.verify_token(uuid, username)
        .await
        .map_err(|err| match err {
            VerifyTokenError::NotFound => RefreshError::InvalidRefreshToken,
            VerifyTokenError::Other(message) => {
                eprintln!("refresh error: {}", message);
                RefreshError::SomethingElse
            }
        })
        .map(|result| RefreshOk {
            uuid: result.uuid,
            username: result.username,
            access_token: result.access_token,
            refresh_token: result.refresh_token,
        })
}

#[derive(Serialize)]
pub struct RefreshOk {
    pub uuid: String,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug)]
pub enum RefreshError {
    InvalidRefreshToken,
    SomethingElse,
    InvalidApiKey,
}

#[derive(Debug)]
pub enum AccessTokenError {
    InvalidToken,
    SomethingElse,
    InvalidApiKey,
}
