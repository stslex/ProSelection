use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::{handlers::auth::refresh::RefreshError, utils::jwt_util::JwtDecoder};

use super::{ApiKeyParcer, RefreshToken, TokenParser};

impl<'a, 'r> FromRequest<'a, 'r> for RefreshToken {
    type Error = RefreshError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match ApiKeyParcer::parce(request) {
            Ok(_api_key) => {}
            Err(_error) => {
                return Outcome::Failure((Status::Unauthorized, RefreshError::InvalidApiKey))
            }
        }
        let token = match TokenParser::get_token(request) {
            Some(token) => token,
            None => {
                return Outcome::Failure((Status::Unauthorized, RefreshError::InvalidRefreshToken))
            }
        };
        let binding = token.as_str();
        match JwtDecoder::decode_refresh(&binding) {
            Ok(claims) => Outcome::Success(RefreshToken {
                uuid: claims.uuid,
                username: claims.username,
            }),
            Err(error) => {
                log::error!("Invalid refresh token: {}", error);
                Outcome::Failure((Status::Unauthorized, RefreshError::InvalidRefreshToken))
            }
        }
    }
}
