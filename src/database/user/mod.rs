use self::{
    user_db::GetByUuidError,
    user_objects::{user::User, UserCommonOutcome},
};

use super::DatabaseResponse;

pub mod user_db;
pub mod user_objects;

pub trait UserDatabase {
    fn get_user_count(&self) -> UserCommonOutcome<String>;
    fn get_user(&self, uuid: &str) -> Result<User, GetByUuidError>;
    fn get_user_by_username(&self, username: &str) -> Result<User, GetByUuidError>;
    fn get_favourites_count(&self, uuid: &str) -> Result<i64, GetByUuidError>;
    fn get_followers_count(&self, uuid: &str) -> Result<i64, GetByUuidError>;
    fn get_following_count(&self, uuid: &str) -> Result<i64, GetByUuidError>;
    fn follow_user(
        &self,
        follower_uuid: &str,
        followed_uuid: &str,
    ) -> DatabaseResponse<FollowError>;
    fn un_follow_user(
        &self,
        follower_uuid: &str,
        followed_uuid: &str,
    ) -> DatabaseResponse<FollowError>;
    fn is_following(&self, follower_uuid: &str, followed_uuid: &str) -> Result<bool, FollowError>;
    fn add_favourite(&self, uuid: &str, favourite_uuid: &str) -> DatabaseResponse<FavouriteError>;
    fn remove_favourite(
        &self,
        uuid: &str,
        favourite_uuid: &str,
    ) -> DatabaseResponse<FavouriteError>;
    fn is_favourite(&self, uuid: &str, favourite_uuid: &str) -> Result<bool, FavouriteError>;
}

#[derive(Debug)]
pub enum FavouriteError {
    UuidInvalid,
    UserNotFound,
    Conflict,
    InternalError,
}

impl std::fmt::Display for FavouriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FavouriteError::UuidInvalid => write!(f, "UuidInvalid"),
            FavouriteError::UserNotFound => write!(f, "UserNotFound"),
            FavouriteError::Conflict => write!(f, "Conflict"),
            FavouriteError::InternalError => write!(f, "InternalError"),
        }
    }
}

impl std::fmt::Display for DatabaseResponse<FavouriteError> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseResponse::Ok => write!(f, "Ok"),
            DatabaseResponse::Err(err) => write!(f, "Err: {}", err),
        }
    }
}

#[derive(Debug)]
pub enum FollowError {
    UuidInvalid,
    UserNotFound,
    Conflict,
    InternalError,
}

impl std::fmt::Display for FollowError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FollowError::UuidInvalid => write!(f, "UuidInvalid"),
            FollowError::UserNotFound => write!(f, "UserNotFound"),
            FollowError::Conflict => write!(f, "Conflict"),
            FollowError::InternalError => write!(f, "InternalError"),
        }
    }
}

impl std::fmt::Display for DatabaseResponse<FollowError> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseResponse::Ok => write!(f, "Ok"),
            DatabaseResponse::Err(err) => write!(f, "Err: {}", err),
        }
    }
}
