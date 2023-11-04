use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use jwt::{Error, SignWithKey};
use sha2::Sha256;
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
        // todo!("take from config")
        let some_secret = b"some-secret";
        let key: Hmac<Sha256> = Hmac::new_from_slice(some_secret)?;

        let mut claims = BTreeMap::new();
        claims.insert("sub", "someone");

        claims.sign_with_key(&key)
    }
}
