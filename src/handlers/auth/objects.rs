use serde::Serialize;

#[derive(Serialize)]
pub struct LoginOk {
    pub uuid: String,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, PartialEq)]
pub enum LoginError {
    NotFound,
    Other,
}

impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoginError::NotFound => write!(f, "Login not found"),
            LoginError::Other => write!(f, "Other login error"),
        }
    }
}
