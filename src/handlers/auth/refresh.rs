use crate::database::{
    self,
    auth::{self, AuthorizationDatabase},
};
use serde::Serialize;

pub fn refresh(uuid: &str, username: &str, db: database::Conn) -> Result<RefreshOk, RefreshError> {
    match db.verify_token(uuid, username) {
        auth::VerifyTokenOutcome::Ok(result) => Ok(RefreshOk {
            uuid: result.uuid,
            username: result.username,
            access_token: result.access_token,
            refresh_token: result.refresh_token,
        }),
        auth::VerifyTokenOutcome::NotFound => Err(RefreshError::InvalidRefreshToken),
        auth::VerifyTokenOutcome::Other => Err(RefreshError::SomethingElse),
    }
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
