use super::AuthorizationOk;

#[derive(Debug, Clone)]
pub struct RegistrationData<'a> {
    pub login: &'a str,
    pub password: &'a str,
    pub username: &'a str,
}

#[derive(Debug)]
pub enum RegistrationOutcome {
    Ok(AuthorizationOk),
    AlreadyInUse,
    Other(String),
}
