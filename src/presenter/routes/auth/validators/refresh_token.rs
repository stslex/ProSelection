use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::presenter::handlers::auth::refresh::RefreshError;

use super::{jwt_decoder::JwtDecoder, ApiKeyParcer, RefreshToken, TokenParser};

#[async_trait]
impl<'r> FromRequest<'r> for RefreshToken {
    type Error = RefreshError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match ApiKeyParcer::parce(request) {
            Ok(_api_key) => {}
            Err(_error) => {
                return Outcome::Error((Status::Unauthorized, RefreshError::InvalidApiKey))
            }
        }
        let token = match TokenParser::get_token(request) {
            Some(token) => token,
            None => {
                return Outcome::Error((Status::Unauthorized, RefreshError::InvalidRefreshToken))
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
                Outcome::Error((Status::Unauthorized, RefreshError::InvalidRefreshToken))
            }
        }
    }
}
