use crate::{
    data::database::{
        follow::objects::UserSearchError,
        user::{objects::UserDataError, UserDatabase},
    },
    presenter::handlers::user::search::UserSearchRequest,
    utils::Mapper,
    Conn,
};

use super::{objects::UserDataResponse, UserRepository};

#[async_trait]
impl UserRepository for Conn {
    async fn get_user_count(&self) -> Result<String, UserDataError> {
        UserDatabase::get_user_count(self).await
    }
    async fn get_user<'a>(&self, uuid: &'a str) -> Result<UserDataResponse, UserDataError> {
        match UserDatabase::get_user(self, uuid).await {
            Ok(user) => Ok(user.map().await),
            Err(e) => Err(e),
        }
    }
    async fn search_users<'a>(
        &self,
        request: &'a UserSearchRequest<'a>,
    ) -> Result<Vec<UserDataResponse>, UserSearchError> {
        match UserDatabase::search_users(self, request).await {
            Ok(users) => Ok(users.map().await),
            Err(e) => Err(e),
        }
    }
    async fn get_user_by_username<'a>(
        &self,
        username: &'a str,
    ) -> Result<UserDataResponse, UserDataError> {
        match UserDatabase::get_user_by_username(self, username).await {
            Ok(user) => Ok(user.map().await),
            Err(e) => Err(e),
        }
    }
}
