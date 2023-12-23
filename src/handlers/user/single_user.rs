use serde::Serialize;

use crate::database::{
    self,
    user::{user_db::GetByUuidError, user_objects::user::User, UserDatabase},
};

pub async fn get_user<'a>(uuid: &'a str, db: database::Conn) -> Result<UserResponse, UserError> {
    match db.get_user(uuid) {
        Ok(user) => Ok(map_user_info(&user, db)),
        Err(err) => match err {
            GetByUuidError::UuidInvalid => Err(UserError::UuidInvalid),
            GetByUuidError::InternalError => Err(UserError::Other),
        },
    }
}

pub async fn get_user_by_username<'a>(
    username: &'a str,
    db: database::Conn,
) -> Result<UserResponse, UserError> {
    match db.get_user_by_username(username) {
        Ok(user) => Ok(map_user_info(&user, db)),
        Err(err) => match err {
            GetByUuidError::UuidInvalid => Err(UserError::UuidInvalid),
            GetByUuidError::InternalError => Err(UserError::Other),
        },
    }
}

async fn map_user_info(user: &User, db: database::Conn) -> UserResponse {
    UserResponse {
        uuid: user.id.to_string(),
        username: user.username.clone(),
        bio: user.bio.clone(),
        avatar_url: user.avatar_url.clone(),
        followers_count: match db.get_followers_count(&user.id.to_string()) {
            Ok(count) => count,
            Err(err) => {
                eprintln!("Error getting followers count: {}", err);
                0
            }
        },
        following_count: match db.get_following_count(&user.id.to_string()) {
            Ok(count) => count,
            Err(err) => {
                eprintln!("Error getting following count: {}", err);
                0
            }
        },
        favourites_count: match db.get_favourites_count(&user.id.to_string()) {
            Ok(count) => count,
            Err(err) => {
                eprintln!("Error getting favourites count: {}", err);
                0
            }
        },
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    pub uuid: String,
    pub username: String,
    pub avatar_url: String,
    pub bio: String,
    pub followers_count: i64,
    pub following_count: i64,
    pub favourites_count: i64,
}

#[derive(Serialize)]
pub struct IsFavouriteResponse {
    pub is_favourite: bool,
}

#[derive(Serialize)]
pub struct IsFollowingResponse {
    pub is_following: bool,
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
