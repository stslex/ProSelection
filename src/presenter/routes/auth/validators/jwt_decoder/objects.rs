pub struct JwtDecoderResult {
    pub uuid: String,
    pub username: String,
}

#[derive(Debug)]
pub enum JwtDecoderError {
    InvalidEnvSecret,
    InvalidSignature,
    ExpiredSignature,
    ParceError(String),
}

impl std::fmt::Display for JwtDecoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JwtDecoderError::InvalidEnvSecret => write!(f, "Invalid env secret"),
            JwtDecoderError::InvalidSignature => write!(f, "Invalid signature"),
            JwtDecoderError::ExpiredSignature => write!(f, "Expired signature"),
            JwtDecoderError::ParceError(message) => write!(f, "Parce error: {}", message),
        }
    }
}

impl PartialEq for JwtDecoderError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (JwtDecoderError::InvalidEnvSecret, JwtDecoderError::InvalidEnvSecret) => true,
            (JwtDecoderError::InvalidSignature, JwtDecoderError::InvalidSignature) => true,
            (JwtDecoderError::ExpiredSignature, JwtDecoderError::ExpiredSignature) => true,
            (JwtDecoderError::ParceError(message1), JwtDecoderError::ParceError(message2)) => {
                message1 == message2
            }
            _ => false,
        }
    }
}
