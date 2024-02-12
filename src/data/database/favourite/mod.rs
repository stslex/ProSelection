use self::objects::{FavouriteDataSearchRequest, FavouriteEntityResponse};

use super::DatabaseResponse;
mod favourite_db;
pub mod objects;

#[async_trait]
pub trait UserFavouritesDatabase {
    async fn get_favourites_count<'a>(&self, uuid: &'a str) -> Result<i64, FavouriteDbError>;
    async fn add_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
        title: &'a str,
    ) -> Result<(), FavouriteDbError>;
    async fn remove_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> Result<(), FavouriteDbError>;
    async fn is_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> Result<bool, FavouriteDbError>;
    async fn get_user_favourites<'a>(
        &self,
        request: &'a FavouriteDataSearchRequest<'a>,
    ) -> Result<Vec<FavouriteEntityResponse>, FavouriteDbError>;
}

#[derive(Debug, Clone)]
pub enum FavouriteDbError {
    UuidInvalid,
    UserNotFound,
    Conflict,
    InternalError,
}

impl std::fmt::Display for FavouriteDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FavouriteDbError::UuidInvalid => write!(f, "UuidInvalid"),
            FavouriteDbError::UserNotFound => write!(f, "UserNotFound"),
            FavouriteDbError::Conflict => write!(f, "Conflict"),
            FavouriteDbError::InternalError => write!(f, "InternalError"),
        }
    }
}

impl std::fmt::Display for DatabaseResponse<FavouriteDbError> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseResponse::Ok => write!(f, "Ok"),
            DatabaseResponse::Err(err) => write!(f, "Err: {}", err),
        }
    }
}
