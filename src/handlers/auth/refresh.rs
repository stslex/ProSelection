use serde::Serialize;

use crate::{
    database,
    utils::{self},
};

pub fn refresh(refresh_token: &str, db: database::Conn) -> Result<RefreshOk, RefreshError> {
    match utils::JwtDecoder::decode(&refresh_token) {
        Ok(claims) => {
            todo!("Implement refresh")
        }
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
