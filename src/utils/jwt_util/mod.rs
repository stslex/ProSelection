use self::{
    decoder::JwtDecoderError,
    generator::JwtGeneratorError,
    objects::{JwtDecoderResult, JwtResult},
};

pub mod decoder;
pub mod generator;
pub mod objects;
mod tests;

pub trait JwtGenerator {
    fn generate(&self) -> Result<JwtResult, JwtGeneratorError>;
    fn generate_access(&self) -> Result<String, JwtGeneratorError>;
    fn generate_refresh(&self) -> Result<String, JwtGeneratorError>;
    fn generate_token(&self, secret: &[u8], exp_days: i64) -> Result<String, JwtGeneratorError>;
}

pub trait JwtDecoder {
    fn decode_refresh(&self) -> Result<JwtDecoderResult, JwtDecoderError>;
    fn decode_access(&self) -> Result<JwtDecoderResult, JwtDecoderError>;
    fn decode(&self, secret: &[u8]) -> Result<JwtDecoderResult, JwtDecoderError>;
}
