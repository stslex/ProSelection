use std::{collections::BTreeMap, env};

use chrono::Utc;
use hmac::{digest::KeyInit, Hmac};
use jwt::{Error, Header, Token, VerifyWithKey};
use sha2::Sha256;

use super::{objects::JwtDecoderResult, JwtDecoder};

impl JwtDecoder for &str {
    fn decode_access(&self) -> Result<JwtDecoderResult, Error> {
        let env_secret = env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET not found");
        let secret = env_secret.as_bytes();
        self.decode(secret)
    }

    fn decode_refresh(&self) -> Result<JwtDecoderResult, Error> {
        let env_secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET not found");
        let secret = env_secret.as_bytes();
        self.decode(secret)
    }

    fn decode(&self, secret: &[u8]) -> Result<JwtDecoderResult, Error> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(secret).expect("Failed to create key");

        let token: Token<Header, BTreeMap<String, String>, _> = self.verify_with_key(&key)?;
        let claims = token.claims();

        let current_time = Utc::now().timestamp();

        let exp_time = claims.get("exp_time").unwrap().parse::<i64>().unwrap();
        if current_time > exp_time {
            log::error!(
                "Token expired, current time: {}, exp_time: {}",
                current_time,
                exp_time
            );
            return Err(Error::InvalidSignature);
        }
        let result = JwtDecoderResult {
            uuid: claims.get("uuid").unwrap().to_string(),
            username: claims.get("username").unwrap().to_string(),
        };
        Ok(result)
    }
}
