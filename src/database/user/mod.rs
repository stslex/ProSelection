use crate::handlers::user::search::{UserPagingRequest, UserSearchError, UserSearchRequest};

use self::{
    user_db::GetByUuidError,
    user_objects::{user::User, Favourite, Follower, UserCommonOutcome},
};

use super::DatabaseResponse;

pub mod user_db;
pub mod user_objects;

#[async_trait]
pub trait UserDatabase {
    async fn get_user_count(&self) -> UserCommonOutcome<String>;
    async fn get_user(&self, uuid: &str) -> Result<User, GetByUuidError>;
    async fn search_users(&self, request: &UserSearchRequest)
        -> Result<Vec<User>, UserSearchError>;
    async fn get_user_by_username(&self, username: &str) -> Result<User, GetByUuidError>;
    async fn get_favourites_count(&self, uuid: &str) -> Result<i64, GetByUuidError>;
    async fn get_followers_count(&self, uuid: &str) -> Result<i64, GetByUuidError>;
    async fn get_following_count(&self, uuid: &str) -> Result<i64, GetByUuidError>;
    async fn follow_user(
        &self,
        follower_uuid: &str,
        followed_uuid: &str,
    ) -> DatabaseResponse<FollowError>;
    async fn un_follow_user(
        &self,
        follower_uuid: &str,
        followed_uuid: &str,
    ) -> DatabaseResponse<FollowError>;
    async fn is_following(
        &self,
        follower_uuid: &str,
        followed_uuid: &str,
    ) -> Result<bool, FollowError>;
    async fn add_favourite(
        &self,
        uuid: &str,
        favourite_uuid: &str,
    ) -> DatabaseResponse<FavouriteError>;
    async fn remove_favourite(
        &self,
        uuid: &str,
        favourite_uuid: &str,
    ) -> DatabaseResponse<FavouriteError>;
    async fn is_favourite(&self, uuid: &str, favourite_uuid: &str) -> Result<bool, FavouriteError>;
    async fn get_user_followers(
        &self,
        request: &UserPagingRequest,
    ) -> Result<Vec<Follower>, UserSearchError>;
    async fn get_user_favourites(
        &self,
        request: &UserPagingRequest,
    ) -> Result<Vec<Favourite>, UserSearchError>;
    async fn get_user_following(
        &self,
        request: &UserPagingRequest,
    ) -> Result<Vec<Follower>, UserSearchError>;
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
