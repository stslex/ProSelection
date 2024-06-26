use crate::data::repository::objects::{PagingDomainRequest, PagingDomainResponse};

use self::objects::{FavouriteDbError, FavouriteEntityResponse};

mod favourite_db;
pub mod objects;
mod tests;

#[async_trait]
pub trait UserFavouritesDatabase {
    async fn get_favourites_count<'a>(&self, uuid: &'a str) -> Result<i64, FavouriteDbError>;
    async fn add_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
        title: &'a str,
    ) -> Result<FavouriteEntityResponse, FavouriteDbError>;
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
        request: PagingDomainRequest<'a>,
    ) -> Result<PagingDomainResponse<FavouriteEntityResponse>, FavouriteDbError>;
}
