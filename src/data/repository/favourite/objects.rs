use uuid::Uuid;

use crate::{
    data::database::favourite::objects::{
        FavouriteDbError, FavouriteDbSearchRequest, FavouriteEntityResponse,
    },
    utils::Mapper,
};
use rocket::futures;

#[derive(Debug, Clone)]
pub enum FavouriteDataError {
    UuidInvalid,
    UserNotFound,
    Conflict,
    InternalError,
}

#[async_trait]
impl Mapper<FavouriteDataError> for FavouriteDbError {
    async fn map(&self) -> FavouriteDataError {
        match self {
            FavouriteDbError::UuidInvalid => FavouriteDataError::UuidInvalid,
            FavouriteDbError::UserNotFound => FavouriteDataError::UserNotFound,
            FavouriteDbError::Conflict => FavouriteDataError::Conflict,
            FavouriteDbError::InternalError => FavouriteDataError::InternalError,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FavouriteDataResponse {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub favourite_uuid: Uuid,
    pub title: String,
}

#[async_trait]
impl Mapper<FavouriteDataResponse> for FavouriteEntityResponse {
    async fn map(&self) -> FavouriteDataResponse {
        FavouriteDataResponse {
            uuid: self.uuid,
            user_uuid: self.user_uuid,
            favourite_uuid: self.favourite_uuid,
            title: self.title.to_owned(),
        }
    }
}

#[async_trait]
impl Mapper<Vec<FavouriteDataResponse>> for Vec<FavouriteEntityResponse> {
    async fn map(&self) -> Vec<FavouriteDataResponse> {
        futures::future::join_all(self.into_iter().map(|favourite| favourite.map())).await
    }
}
pub struct UserDataSearchRequest<'a> {
    pub request_uuid: &'a str,
    pub uuid: &'a str,
    pub query: &'a str,
    pub page: i64,
    pub page_size: i64,
}

#[async_trait]
impl<'a> Mapper<FavouriteDbSearchRequest<'a>> for UserDataSearchRequest<'a> {
    async fn map(&self) -> FavouriteDbSearchRequest<'a> {
        FavouriteDbSearchRequest {
            request_uuid: self.request_uuid,
            uuid: self.uuid,
            query: self.query,
            page: self.page,
            page_size: self.page_size,
        }
    }
}
