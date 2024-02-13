use crate::{
    data::database::{follow::objects::UserSearchError, user::objects::UserDataError},
    presenter::handlers::user::search::UserSearchRequest,
};

use self::objects::UserDataResponse;

pub mod objects;
pub mod repository;

#[async_trait]
pub trait UserRepository {
    async fn get_user_count(&self) -> Result<String, UserDataError>;
    async fn get_user<'a>(&self, uuid: &'a str) -> Result<UserDataResponse, UserDataError>;
    async fn search_users<'a>(
        &self,
        request: &'a UserSearchRequest<'a>,
    ) -> Result<Vec<UserDataResponse>, UserSearchError>;
    async fn get_user_by_username<'a>(
        &self,
        username: &'a str,
    ) -> Result<UserDataResponse, UserDataError>;
}
