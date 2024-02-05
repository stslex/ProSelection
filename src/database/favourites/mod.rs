use super::DatabaseResponse;
mod favourite_db;

#[async_trait]
pub trait UserFavouritesDatabase {
    async fn get_favourites_count<'a>(&self, uuid: &'a str) -> Result<i64, FavouriteError>;
    async fn add_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
        title: &'a str,
    ) -> Result<(), FavouriteError>;
    async fn remove_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> Result<(), FavouriteError>;
    async fn is_favourite(&self, uuid: &str, favourite_uuid: &str) -> Result<bool, FavouriteError>;
}

#[derive(Debug, Clone)]
pub enum FavouriteError {
    UuidInvalid,
    UserNotFound,
    Conflict,
    InternalError,
}

impl std::fmt::Display for FavouriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FavouriteError::UuidInvalid => write!(f, "UuidInvalid"),
            FavouriteError::UserNotFound => write!(f, "UserNotFound"),
            FavouriteError::Conflict => write!(f, "Conflict"),
            FavouriteError::InternalError => write!(f, "InternalError"),
        }
    }
}

impl std::fmt::Display for DatabaseResponse<FavouriteError> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseResponse::Ok => write!(f, "Ok"),
            DatabaseResponse::Err(err) => write!(f, "Err: {}", err),
        }
    }
}
