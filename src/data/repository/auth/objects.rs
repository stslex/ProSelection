#[derive(Debug)]
pub enum AuthDataError {
    NotFound,
    InvalidPassword,
    Other,
}

#[derive(Debug)]
pub struct AuthDataResponse {
    pub uuid: String,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug)]
pub enum RegDataError {
    AlreadyInUse,
    Other(String),
}

#[derive(Debug)]
pub enum VerifyTokenError {
    NotFound,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct RegistrationData<'a> {
    pub login: &'a str,
    pub password: &'a str,
    pub username: &'a str,
}
