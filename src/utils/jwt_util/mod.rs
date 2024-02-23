use self::decoder::{JwtDecoderError, JwtDecoderResult};

pub mod decoder;

mod tests;

pub trait JwtDecoder {
    fn decode_refresh(&self) -> Result<JwtDecoderResult, JwtDecoderError>;
    fn decode_access(&self) -> Result<JwtDecoderResult, JwtDecoderError>;
    fn decode(&self, secret: &[u8]) -> Result<JwtDecoderResult, JwtDecoderError>;
}
