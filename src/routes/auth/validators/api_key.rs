use super::{ApiKey, ApiKeyParcer};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
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
