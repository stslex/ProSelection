use jwt::Error;

use self::jwt_objects::{JwtDecoderResult, JwtResult};

pub mod jwt_objects;
pub mod jwt_utils;

pub trait JwtUtil {
    fn generate(&self) -> Result<JwtResult, Error>;
    fn generate_access(&self) -> Result<String, Error>;
    fn generate_refresh(&self) -> Result<String, Error>;
    fn generate_token(&self, secret: &[u8], exp_days: i64) -> Result<String, Error>;
}

pub trait JwtDecoder {
    fn decode_refresh(&self) -> Result<JwtDecoderResult, Error>;
    fn decode_access(&self) -> Result<JwtDecoderResult, Error>;
    fn decode(&self, secret: &[u8]) -> Result<JwtDecoderResult, Error>;
}
