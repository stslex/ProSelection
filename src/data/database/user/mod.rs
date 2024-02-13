use crate::presenter::handlers::user::search::UserSearchRequest;

use self::objects::{UserCreateDataError, UserDataError, UserEntity, UserEntityCreate};
use super::follow::objects::UserSearchError;

pub mod objects;
pub mod user_db;

#[async_trait]
pub trait UserDatabase {
    async fn get_user_count(&self) -> Result<String, UserDataError>;
    async fn get_user<'a>(&self, uuid: &'a str) -> Result<UserEntity, UserDataError>;
    async fn get_user_by_login<'a>(&self, login: &'a str) -> Result<UserEntity, UserDataError>;
    async fn insert_user<'a>(
        &self,
        user: UserEntityCreate,
    ) -> Result<UserEntity, UserCreateDataError>;
    async fn search_users<'a>(
        &self,
        request: &'a UserSearchRequest<'a>,
    ) -> Result<Vec<UserEntity>, UserSearchError>;
    async fn get_user_by_username<'a>(
        &self,
        username: &'a str,
    ) -> Result<UserEntity, UserDataError>;
}
