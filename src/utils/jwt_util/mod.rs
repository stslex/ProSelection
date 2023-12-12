use jwt::Error;

use self::objects::{JwtDecoderResult, JwtResult};

pub mod decoder;
pub mod generator;
pub mod objects;

mod test_decoder;
mod test_generator;

pub trait JwtGenerator {
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
