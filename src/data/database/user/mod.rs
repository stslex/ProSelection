use crate::presenter::handlers::user::search::UserSearchRequest;

use self::{
    user_db::GetByUuidError,
    user_objects::{user::User, UserCommonOutcome},
};

use super::follow::objects::{FollowDataError, UserSearchError};

pub mod user_db;
pub mod user_objects;

#[async_trait]
pub trait UserDatabase {
    async fn get_user_count(&self) -> UserCommonOutcome<String>;
    async fn get_user(&self, uuid: &str) -> Result<User, GetByUuidError>;
    async fn search_users(&self, request: &UserSearchRequest)
        -> Result<Vec<User>, UserSearchError>;
    async fn get_user_by_username(&self, username: &str) -> Result<User, GetByUuidError>;
}

impl std::fmt::Display for FollowDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FollowDataError::UuidInvalid => write!(f, "UuidInvalid"),
            FollowDataError::UserNotFound => write!(f, "UserNotFound"),
            FollowDataError::Conflict => write!(f, "Conflict"),
            FollowDataError::InternalError => write!(f, "InternalError"),
        }
    }
}
