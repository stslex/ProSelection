use uuid::Uuid;

use crate::data::{
    database::favourite::objects::{FavouriteDbError, FavouriteEntityResponse},
    repository::objects::PagingDomainResponse,
};

#[derive(Debug, Clone)]
pub enum FavouriteDataError {
    UuidInvalid,
    UserNotFound,
    Conflict,
    InternalError,
}

impl Into<FavouriteDataError> for FavouriteDbError {
    fn into(self) -> FavouriteDataError {
        match self {
            FavouriteDbError::UuidInvalid => FavouriteDataError::UuidInvalid,
            FavouriteDbError::UserNotFound => FavouriteDataError::UserNotFound,
            FavouriteDbError::Conflict => FavouriteDataError::Conflict,
            FavouriteDbError::InternalError => FavouriteDataError::InternalError,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FavouriteDataResponse {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub favourite_uuid: Uuid,
    pub title: String,
}

impl Into<FavouriteDataResponse> for FavouriteEntityResponse {
    fn into(self) -> FavouriteDataResponse {
        FavouriteDataResponse {
            uuid: self.uuid,
            user_uuid: self.user_uuid,
            favourite_uuid: self.favourite_uuid,
            title: self.title.to_owned(),
        }
    }
}

impl Into<PagingDomainResponse<FavouriteDataResponse>>
    for PagingDomainResponse<FavouriteEntityResponse>
{
    fn into(self) -> PagingDomainResponse<FavouriteDataResponse> {
        PagingDomainResponse {
            page: self.page,
            page_size: self.page_size,
            total: self.total,
            has_more: self.has_more,
            result: self.result.into_iter().map(|v| v.into()).collect(),
        }
    }
}
