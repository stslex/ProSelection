use std::env;

use rocket::Request;

use super::{api_key::ApiKeyError, ApiKey, ApiKeyParcer, TokenParser};

const AUTH_HEADER: &str = "Authorization";
const BEARER_PREFIX: &str = "Bearer ";

impl<'r> TokenParser for Request<'r> {
    fn get_token(&self) -> Option<String> {
        let token = self.headers().get_one(AUTH_HEADER);
        match token {
            Some(token) => {
                let replaced_token = token.replace(BEARER_PREFIX, "");
                Some(replaced_token)
            }
            None => None,
        }
    }
}

const API_KEY_HEADER: &str = "X-Api-Key";
const API_KEY_ENV_VAR: &str = "API_KEY";

impl<'a> ApiKeyParcer for Request<'a> {
    fn parce(&self) -> Result<ApiKey, ApiKeyError> {
        match self.headers().get_one(API_KEY_HEADER) {
            Some(key) => match key == env::var(API_KEY_ENV_VAR).unwrap() {
                true => Ok(ApiKey {
                    key: key.to_string(),
                }),
                false => Err(ApiKeyError::InvalidApiKey),
            },
            None => Err(ApiKeyError::MissingApiKey),
        }
    }
}
