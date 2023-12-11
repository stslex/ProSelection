use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::{handlers::auth::refresh::TokenError, utils};

pub struct AccessToken {
    pub uuid: String,
    pub username: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for AccessToken {
    type Error = TokenError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("access_token");
        match token {
            Some(token) => match utils::JwtDecoder::decode(&token) {
                Ok(claims) => Outcome::Success(AccessToken {
                    uuid: claims.uuid,
                    username: claims.username,
                }),
                Err(_) => Outcome::Failure((Status::Unauthorized, TokenError::InvalidToken)),
            },
            None => Outcome::Failure((Status::Unauthorized, TokenError::SomethingElse)),
        }
    }
}