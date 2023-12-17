use serde::Serialize;

use crate::database::{
    self,
    user::{user_db::GetUserError, UserDatabase},
};

pub fn get_user<'a>(uuid: &'a str, db: database::Conn) -> Result<UserResponse, UserError> {
    match db.get_user(uuid) {
        Ok(user) => Ok(UserResponse {
            uuid: user.id.to_string(),
            username: user.username,
        }),
        Err(err) => match err {
            GetUserError::UuidInvalid => Err(UserError::UuidInvalid),
            GetUserError::InternalError => Err(UserError::Other),
        },
    }
}

pub fn get_user_by_username<'a>(
    username: &'a str,
    db: database::Conn,
) -> Result<UserResponse, UserError> {
    match db.get_user_by_username(username) {
        Ok(user) => Ok(UserResponse {
            uuid: user.id.to_string(),
            username: user.username,
        }),
        Err(err) => match err {
            GetUserError::UuidInvalid => Err(UserError::UuidInvalid),
            GetUserError::InternalError => Err(UserError::Other),
        },
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    pub uuid: String,
    pub username: String,
}

pub enum UserError {
    UuidInvalid,
    Other,
}

impl std::fmt::Debug for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserError::UuidInvalid => write!(f, "UuidInvalid"),
            UserError::Other => write!(f, "Other"),
        }
    }
}
