use std::{collections::BTreeMap, env};

use chrono::Duration;
use hmac::{Hmac, Mac};
use jwt::{Error, SignWithKey};
use sha2::Sha256;

use super::{
    objects::{JwtObject, JwtResult},
    JwtGenerator,
};

const ACCESS_EXP_TIME_DAYS: i64 = 7;
const REFRESH_EXP_TIME_DAYS: i64 = 30;

impl JwtGenerator for JwtObject {
    fn generate(&self) -> Result<JwtResult, Error> {
        let access_token = self.generate_access()?;
        let refresh_token = self.generate_refresh()?;
        Ok(JwtResult {
            access_token: access_token,
            refresh_token: refresh_token,
        })
    }

    fn generate_access(&self) -> Result<String, Error> {
        let env_secret = env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET not found");
        let secret = env_secret.as_bytes();
        self.generate_token(secret, ACCESS_EXP_TIME_DAYS)
    }

    fn generate_refresh(&self) -> Result<String, Error> {
        let env_secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET not found");
        let secret = env_secret.as_bytes();
        self.generate_token(secret, REFRESH_EXP_TIME_DAYS)
    }

    fn generate_token(&self, env_secret: &[u8], exp_days: i64) -> Result<String, Error> {
        log::info!("Generating token for user: {}", self.username);

        let days: Duration = match std::panic::catch_unwind(|| Duration::days(exp_days)) {
            Ok(result) => result,
            Err(_) => {
                log::error!("Failed to create duration");
                return Err(Error::InvalidSignature);
            }
        };
        let exp_time = chrono::Utc::now()
            .checked_add_signed(days)
            .expect("Failed to add days")
            .timestamp();
        log::info!("exp_time: {}", exp_time);

        let key: Hmac<Sha256> = Hmac::new_from_slice(env_secret).expect("Failed to create key");
        let mut claims = BTreeMap::new();
        claims.insert("uuid", self.uuid.to_string());
        claims.insert("username", self.username.clone());
        claims.insert("exp_time", exp_time.to_string());
        claims.sign_with_key(&key)
    }
}
