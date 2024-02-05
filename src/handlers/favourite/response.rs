use crate::{
    handlers::objects::response::{
        ErrorResponse, ERROR_FAVOURITE_CONFLICT, ERROR_FAVOURITE_USER_NOT_FOUND,
        ERROR_FAVOURITE_UUID_INVALID, ERROR_UNKNOWN,
    },
    repository::favourite::objects::FavouriteDataError,
    utils::Mapper,
};

#[async_trait]
impl Mapper<&'static ErrorResponse<'static>> for FavouriteDataError {
    async fn map(&self) -> &'static ErrorResponse<'static> {
        eprint!("Error: {:?}", &self);
        match self {
            FavouriteDataError::UuidInvalid => ERROR_FAVOURITE_UUID_INVALID,
            FavouriteDataError::UserNotFound => ERROR_FAVOURITE_USER_NOT_FOUND,
            FavouriteDataError::Conflict => ERROR_FAVOURITE_CONFLICT,
            FavouriteDataError::InternalError => ERROR_UNKNOWN,
        }
    }
}
