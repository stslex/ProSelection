use crate::{
    database::{
        self,
        auth::{self, AuthorizationDatabase},
    },
    utils::{self, jwt_objects::JwtMapper, JwtUtil},
};
use serde::Serialize;

pub fn refresh(refresh_token: &str, db: database::Conn) -> Result<RefreshOk, RefreshError> {
    match utils::JwtDecoder::decode(&refresh_token) {
        Ok(claims) => match db.verify_token(&claims.uuid, &claims.username) {
            auth::VerifyTokenOutcome::Ok(_) => match claims.map().generate() {
                Ok(token_res) => Ok(RefreshOk {
                    access_token: token_res.access_token,
                    refresh_token: token_res.refresh_token,
                }),
                Err(_) => Err(RefreshError::SomethingElse),
            },
            auth::VerifyTokenOutcome::NotFound => Err(RefreshError::InvalidRefreshToken),
            auth::VerifyTokenOutcome::Other => Err(RefreshError::SomethingElse),
        },
        Err(_) => Err(RefreshError::InvalidRefreshToken),
    }
}

#[derive(Serialize)]
pub struct RefreshOk {
    pub access_token: String,
    pub refresh_token: String,
}

pub enum RefreshError {
    InvalidRefreshToken,
    SomethingElse,
}
