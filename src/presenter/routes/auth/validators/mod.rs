use self::api_key::ApiKeyError;

pub mod access_token;
mod api_key;
mod jwt_decoder;
pub mod refresh_token;
mod token_parser;

trait TokenParser {
    fn get_token(&self) -> Option<String>;
}

trait ApiKeyParcer {
    fn parce(&self) -> Result<ApiKey, ApiKeyError>;
}

pub struct AccessToken {
    pub uuid: String,
    pub username: String,
}

pub struct RefreshToken {
    pub uuid: String,
    pub username: String,
}

pub struct ApiKey {
    pub key: String,
}
