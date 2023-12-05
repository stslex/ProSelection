use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use jwt::{Error, SignWithKey};
use sha2::Sha256;
use std::env;
use uuid::Uuid;

use crate::database::user::user_objects::user::User;

pub struct JwtObject {
    pub uuid: Uuid,
    pub username: String,
}

pub trait JwtMapper {
    fn map(&self) -> JwtObject;
}

impl JwtMapper for User {
    fn map(&self) -> JwtObject {
        JwtObject {
            uuid: self.id,
            username: self.username.clone(),
        }
    }
}

pub trait JwtUtil {
    fn generate(&self) -> Result<String, Error>;
}

impl JwtUtil for JwtObject {
    fn generate(&self) -> Result<String, Error> {
        let env_secret = env::var("JWT_SECRET").expect("JWT_SECRET not found");
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(env_secret.as_bytes()).expect("Failed to create HMAC key");

        let mut claims = BTreeMap::new();
        claims.insert("uuid", self.uuid);

        // TODO - Add expiration time
        // TOOD - Add issuer
        // TOOD - Add username
        claims.sign_with_key(&key)
    }
}
