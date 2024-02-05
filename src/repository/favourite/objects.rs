use crate::{database::favourites::FavouriteError, utils::Mapper};

#[derive(Debug, Clone)]
pub enum FavouriteDataError {
    UuidInvalid,
    UserNotFound,
    Conflict,
    InternalError,
}

#[async_trait]
impl Mapper<FavouriteDataError> for FavouriteError {
    async fn map(&self) -> FavouriteDataError {
        match self {
            FavouriteError::UuidInvalid => FavouriteDataError::UuidInvalid,
            FavouriteError::UserNotFound => FavouriteDataError::UserNotFound,
            FavouriteError::Conflict => FavouriteDataError::Conflict,
            FavouriteError::InternalError => FavouriteDataError::InternalError,
        }
    }
}
