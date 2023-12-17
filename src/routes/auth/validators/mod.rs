pub mod access_token;
pub mod refresh_token;
mod token_parser;

trait TokenParser {
    fn get_token(&self) -> Option<String>;
}

pub struct AccessToken {
    pub uuid: String,
    pub username: String,
}

pub struct RefreshToken {
    pub uuid: String,
    pub username: String,
}
