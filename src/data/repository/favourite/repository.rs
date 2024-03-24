use crate::{
    data::{
        database::favourite::UserFavouritesDatabase,
        repository::objects::{PagingDomainRequest, PagingDomainResponse},
    },
    Conn,
};

use super::{
    objects::{FavouriteDataError, FavouriteDataResponse},
    FavouriteRepository,
};

#[async_trait]
impl FavouriteRepository for Conn {
    async fn get_favourites_count<'a>(&self, uuid: &'a str) -> Result<i64, FavouriteDataError> {
        match UserFavouritesDatabase::get_favourites_count(self, uuid).await {
            Ok(count) => Ok(count),
            Err(err) => Err(err.into()),
        }
    }
    async fn add_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
        title: &'a str,
    ) -> Result<(), FavouriteDataError> {
        match UserFavouritesDatabase::add_favourite(self, uuid, favourite_uuid, title).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
    async fn remove_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> Result<(), FavouriteDataError> {
        match UserFavouritesDatabase::remove_favourite(self, uuid, favourite_uuid).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
    async fn is_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> Result<bool, FavouriteDataError> {
        match UserFavouritesDatabase::is_favourite(self, uuid, favourite_uuid).await {
            Ok(is_favourite) => Ok(is_favourite),
            Err(err) => Err(err.into()),
        }
    }
    async fn get_user_favourites<'a>(
        &self,
        request: &'a PagingDomainRequest<'a>,
    ) -> Result<PagingDomainResponse<FavouriteDataResponse>, FavouriteDataError> {
        UserFavouritesDatabase::get_user_favourites(self, request.to_owned())
            .await
            .map(|res| res.into())
            .map_err(|err| err.into())
    }
}
