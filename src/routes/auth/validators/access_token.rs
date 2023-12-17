use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::{handlers::auth::refresh::TokenError, utils::jwt_util::JwtDecoder};

use super::{AccessToken, TokenParser};

impl<'a, 'r> FromRequest<'a, 'r> for AccessToken {
    type Error = TokenError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let token = match TokenParser::get_token(request) {
            Some(token) => token,
            None => return Outcome::Failure((Status::Unauthorized, TokenError::InvalidToken)),
        };
        let binding = token.as_str();
        match JwtDecoder::decode_access(&binding) {
            Ok(claims) => Outcome::Success(AccessToken {
                uuid: claims.uuid,
                username: claims.username,
            }),
            Err(error) => {
                log::error!("Invalid access token: {}", error);
                Outcome::Failure((Status::Unauthorized, TokenError::InvalidToken))
            }
        }
    }
}
