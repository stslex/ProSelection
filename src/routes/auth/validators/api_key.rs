use super::{ApiKey, ApiKeyParcer};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

#[async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match ApiKeyParcer::parce(request) {
            Ok(api_key) => Outcome::Success(api_key),
            Err(error) => Outcome::Failure((Status::Unauthorized, error)),
        }
    }
}

#[derive(Debug)]
pub enum ApiKeyError {
    InvalidApiKey,
    MissingApiKey,
}
