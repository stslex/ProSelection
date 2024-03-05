use crate::presenter::handlers::{
    matches::objects::UserMatchError,
    objects::response::{
        ErrorResponse, ERROR_MATCHES_CONFLICT, ERROR_MATCHES_NOT_FOUND, ERROR_MATCHES_UUID_INVALID,
        ERROR_NO_PERMISSION, ERROR_UNKNOWN,
    },
};

impl Into<&'static ErrorResponse<'static>> for UserMatchError {
    fn into(self) -> &'static ErrorResponse<'static> {
        match self {
            UserMatchError::NoPermission => ERROR_NO_PERMISSION,
            UserMatchError::InternalError => ERROR_UNKNOWN,
            UserMatchError::UuidInvalid => ERROR_MATCHES_UUID_INVALID,
            UserMatchError::MatchesNotFound => ERROR_MATCHES_NOT_FOUND,
            UserMatchError::MatchesNotCreated => ERROR_MATCHES_CONFLICT,
        }
    }
}
