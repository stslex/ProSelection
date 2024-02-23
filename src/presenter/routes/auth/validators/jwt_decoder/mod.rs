use self::objects::{JwtDecoderError, JwtDecoderResult};

mod decoder;
pub mod objects;
mod test;

pub trait JwtDecoder {
    fn decode_refresh(&self) -> Result<JwtDecoderResult, JwtDecoderError>;
    fn decode_access(&self) -> Result<JwtDecoderResult, JwtDecoderError>;
    fn decode(&self, secret: &[u8]) -> Result<JwtDecoderResult, JwtDecoderError>;
}
