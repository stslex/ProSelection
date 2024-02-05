use super::AuthorizationOk;

#[derive(Debug, Clone)]
pub struct RegistrationData {
    pub login: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug)]
pub enum RegistrationOutcome {
    Ok(AuthorizationOk),
    AlreadyInUse,
    Other(String),
}
