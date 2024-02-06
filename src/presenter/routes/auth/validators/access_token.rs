use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::{presenter::handlers::auth::refresh::AccessTokenError, utils::jwt_util::JwtDecoder};

use super::{AccessToken, ApiKeyParcer, TokenParser};

#[async_trait]
impl<'r> FromRequest<'r> for AccessToken {
    type Error = AccessTokenError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match ApiKeyParcer::parce(request) {
            Ok(_api_key) => {}
            Err(_error) => {
                return Outcome::Error((Status::Unauthorized, AccessTokenError::InvalidApiKey))
            }
        }
        let token = match TokenParser::get_token(request) {
            Some(token) => token,
            None => return Outcome::Error((Status::Unauthorized, AccessTokenError::InvalidToken)),
        };
        let binding = token.as_str();
        match JwtDecoder::decode_access(&binding) {
            Ok(claims) => Outcome::Success(AccessToken {
                uuid: claims.uuid,
                username: claims.username,
            }),
            Err(error) => {
                log::error!("Invalid access token: {}", error);
                Outcome::Error((Status::Unauthorized, AccessTokenError::InvalidToken))
            }
        }
    }
}
