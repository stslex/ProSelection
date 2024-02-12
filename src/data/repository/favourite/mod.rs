use crate::data::database::favourite::objects::FavouriteDataSearchRequest;

use self::objects::{FavouriteDataError, FavouriteDataResponse};

pub mod objects;
mod repository;

#[async_trait]
pub trait FavouriteRepository {
    async fn get_favourites_count<'a>(&self, uuid: &'a str) -> Result<i64, FavouriteDataError>;
    async fn add_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
        title: &'a str,
    ) -> Result<(), FavouriteDataError>;
    async fn remove_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> Result<(), FavouriteDataError>;
    async fn is_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> Result<bool, FavouriteDataError>;
    async fn get_user_favourites<'a>(
        &self,
        request: &'a FavouriteDataSearchRequest<'a>,
    ) -> Result<Vec<FavouriteDataResponse>, FavouriteDataError>;
}
