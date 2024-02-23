use crate::{data::repository::user::objects::UserDataResponse, utils::Mapper};

pub struct JwtObject {
    pub uuid: String,
    pub username: String,
}

#[async_trait]
impl Mapper<JwtObject> for UserDataResponse {
    async fn map(&self) -> JwtObject {
        JwtObject {
            uuid: self.id.to_string(),
            username: self.username.to_owned(),
        }
    }
}

pub struct JwtResult {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug)]
pub enum JwtGeneratorError {
    InvalidEnvSecret,
    DurationOutOfBound,
    TimeCreationError,
    SignWithKey,
    CreateKey,
}

impl std::fmt::Display for JwtGeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JwtGeneratorError::InvalidEnvSecret => write!(f, "Invalid env secret"),
            JwtGeneratorError::TimeCreationError => write!(f, "Time creation error"),
            JwtGeneratorError::SignWithKey => write!(f, "Sign with key error"),
            JwtGeneratorError::CreateKey => write!(f, "Create key error"),
            JwtGeneratorError::DurationOutOfBound => write!(f, "Duration out of bound"),
        }
    }
}

impl PartialEq for JwtGeneratorError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (JwtGeneratorError::InvalidEnvSecret, JwtGeneratorError::InvalidEnvSecret) => true,
            (JwtGeneratorError::DurationOutOfBound, JwtGeneratorError::DurationOutOfBound) => true,
            (JwtGeneratorError::TimeCreationError, JwtGeneratorError::TimeCreationError) => true,
            (JwtGeneratorError::SignWithKey, JwtGeneratorError::SignWithKey) => true,
            (JwtGeneratorError::CreateKey, JwtGeneratorError::CreateKey) => true,
            _ => false,
        }
    }
}
