use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use jwt::{Error, Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha256;
use std::env;

use super::{
    jwt_objects::{JwtDecoderResult, JwtObject, JwtResult},
    JwtDecoder, JwtUtil,
};

const ACCESS_EXP_TIME_DAYS: i64 = 7;
const REFRESH_EXP_TIME_DAYS: i64 = 30;

impl JwtUtil for JwtObject {
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

    fn generate_token(&self, env_secret: &[u8], exp_time: i64) -> Result<String, Error> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(env_secret).expect("Failed to create key");
        let mut claims = BTreeMap::new();
        claims.insert("uuid", self.uuid.to_string());
        claims.insert("username", self.username.clone());
        claims.insert("exp_time", exp_time.to_string());
        claims.sign_with_key(&key)
    }
}

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

        let current_time = chrono::Utc::now().timestamp();
        let exp_time = claims.get("exp_time").unwrap().parse::<i64>().unwrap();
        if current_time > exp_time {
            return Err(Error::InvalidSignature);
        }
        let result = JwtDecoderResult {
            uuid: claims.get("uuid").unwrap().to_string(),
            username: claims.get("username").unwrap().to_string(),
        };
        Ok(result)
    }
}
