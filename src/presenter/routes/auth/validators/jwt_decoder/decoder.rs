use std::{collections::BTreeMap, env};

use chrono::Utc;
use hmac::{
    digest::{InvalidLength, KeyInit},
    Hmac,
};
use jwt::{Header, Token, VerifyWithKey};
use sha2::Sha256;

use super::{
    objects::{JwtDecoderError, JwtDecoderResult},
    JwtDecoder,
};

impl JwtDecoder for &str {
    fn decode_access(&self) -> Result<JwtDecoderResult, JwtDecoderError> {
        let env_secret = match env::var("JWT_ACCESS_SECRET") {
            Ok(result) => result,
            Err(_) => {
                log::error!("JWT_ACCESS_SECRET not found");
                return Err(JwtDecoderError::InvalidEnvSecret);
            }
        };
        let secret = env_secret.as_bytes();
        self.decode(secret)
    }

    fn decode_refresh(&self) -> Result<JwtDecoderResult, JwtDecoderError> {
        let env_secret = match env::var("JWT_REFRESH_SECRET") {
            Ok(result) => result,
            Err(_) => {
                log::error!("JWT_REFRESH_SECRET not found");
                return Err(JwtDecoderError::InvalidEnvSecret);
            }
        };
        let secret = env_secret.as_bytes();
        self.decode(secret)
    }

    fn decode(&self, secret: &[u8]) -> Result<JwtDecoderResult, JwtDecoderError> {
        let key: Hmac<Sha256> = match Hmac::new_from_slice(secret) {
            Ok(result) => result,
            Err(InvalidLength) => {
                log::error!("Failed to create key");
                return Err(JwtDecoderError::InvalidEnvSecret);
            }
        };

        let token: Token<Header, BTreeMap<String, String>, _> = match self.verify_with_key(&key) {
            Ok(result) => result,
            Err(_) => {
                log::error!("Failed to verify token");
                return Err(JwtDecoderError::InvalidSignature);
            }
        };
        let claims = token.claims();

        let current_time = Utc::now().timestamp();

        let exp_time = match claims.get("exp_time") {
            Some(result) => match result.parse::<i64>() {
                Ok(result) => result,
                Err(_) => {
                    log::error!("Failed to parse exp_time not a number");
                    return Err(JwtDecoderError::ParceError(
                        "exp_time not a number".to_string(),
                    ));
                }
            },
            None => {
                log::error!("Failed to get exp_time");
                return Err(JwtDecoderError::ParceError("exp_time".to_string()));
            }
        };

        if current_time > exp_time {
            log::error!(
                "Token expired, current time: {}, exp_time: {}",
                current_time,
                exp_time
            );
            return Err(JwtDecoderError::ExpiredSignature);
        }
        Ok(JwtDecoderResult {
            uuid: claims.get("uuid").unwrap().to_string(),
            username: claims.get("username").unwrap().to_string(),
        })
    }
}
