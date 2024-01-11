use std::{collections::BTreeMap, env};

use chrono::Duration;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

use super::{
    objects::{JwtObject, JwtResult},
    JwtGenerator,
};

const ACCESS_EXP_TIME_DAYS: i64 = 7;
const REFRESH_EXP_TIME_DAYS: i64 = 30;

#[async_trait]
impl JwtGenerator for JwtObject {
    async fn generate(&self) -> Result<JwtResult, JwtGeneratorError> {
        Ok(JwtResult {
            access_token: self.generate_access().await?,
            refresh_token: self.generate_refresh().await?,
        })
    }

    async fn generate_access(&self) -> Result<String, JwtGeneratorError> {
        let env_secret = match env::var("JWT_ACCESS_SECRET") {
            Ok(result) => result,
            Err(_) => {
                log::error!("JWT_ACCESS_SECRET not found");
                return Err(JwtGeneratorError::InvalidEnvSecret);
            }
        };
        let secret = env_secret.as_bytes();
        self.generate_token(secret, ACCESS_EXP_TIME_DAYS).await
    }

    async fn generate_refresh(&self) -> Result<String, JwtGeneratorError> {
        let env_secret = match env::var("JWT_REFRESH_SECRET") {
            Ok(result) => result,
            Err(_) => {
                log::error!("JWT_REFRESH_SECRET not found");
                return Err(JwtGeneratorError::InvalidEnvSecret);
            }
        };
        let secret = env_secret.as_bytes();
        self.generate_token(secret, REFRESH_EXP_TIME_DAYS).await
    }

    async fn generate_token(
        &self,
        env_secret: &[u8],
        exp_days: i64,
    ) -> Result<String, JwtGeneratorError> {
        log::info!("Generating token for user: {}", self.username);

        let days: Duration = match std::panic::catch_unwind(|| Duration::days(exp_days)) {
            Ok(result) => result,
            Err(_) => {
                log::error!("Failed to create duration / out of bound");
                return Err(JwtGeneratorError::DurationOutOfBound);
            }
        };
        let exp_time = match chrono::Utc::now().checked_add_signed(days) {
            Some(result) => result,
            None => {
                log::error!("Failed to add days");
                return Err(JwtGeneratorError::TimeCreationError);
            }
        }
        .timestamp();

        log::info!("exp_time: {}", exp_time);

        let key: Hmac<Sha256> = match Hmac::new_from_slice(env_secret) {
            Ok(result) => result,
            Err(_) => {
                log::error!("Failed to create key");
                return Err(JwtGeneratorError::CreateKey);
            }
        };

        let mut claims = BTreeMap::new();
        claims.insert("uuid", self.uuid.to_string());
        claims.insert("username", self.username.clone());
        claims.insert("exp_time", exp_time.to_string());

        match claims.sign_with_key(&key) {
            Ok(result) => Ok(result),
            Err(_) => {
                log::error!("Failed to sign with key");
                Err(JwtGeneratorError::SignWithKey)
            }
        }
    }
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
