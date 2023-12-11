use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::{handlers::auth::refresh::RefreshError, utils};

pub struct RefreshToken {
    pub uuid: String,
    pub username: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for RefreshToken {
    type Error = RefreshError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("refresh_token");
        match token {
            Some(token) => match utils::JwtDecoder::decode(&token) {
                Ok(claims) => Outcome::Success(RefreshToken {
                    uuid: claims.uuid,
                    username: claims.username,
                }),
                Err(_) => {
                    Outcome::Failure((Status::Unauthorized, RefreshError::InvalidRefreshToken))
                }
            },
            None => Outcome::Failure((Status::Unauthorized, RefreshError::InvalidRefreshToken)),
        }
    }
}
