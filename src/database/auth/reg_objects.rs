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
    RegistrationFieldValid(RegistrationFieldValidError),
    Other(String),
}

#[derive(Debug)]
pub enum RegistrationFieldValid {
    Ok,
    Error(RegistrationFieldValidError),
}

impl PartialEq for RegistrationFieldValid {
    fn eq(&self, other: &Self) -> bool {
        match self {
            RegistrationFieldValid::Ok => match other {
                RegistrationFieldValid::Ok => true,
                _ => false,
            },
            RegistrationFieldValid::Error(err) => match other {
                RegistrationFieldValid::Error(other_err) => err == other_err,
                _ => false,
            },
        }
    }
}

#[derive(Debug)]
pub enum RegistrationFieldValidError {
    WeakPassword,
    WeakUsername,
    WeakLogin,
    PasswordTooLong,
    EqualLoginPassword,
}

impl PartialEq for RegistrationFieldValidError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            RegistrationFieldValidError::WeakPassword => match other {
                RegistrationFieldValidError::WeakPassword => true,
                _ => false,
            },
            RegistrationFieldValidError::WeakUsername => match other {
                RegistrationFieldValidError::WeakUsername => true,
                _ => false,
            },
            RegistrationFieldValidError::WeakLogin => match other {
                RegistrationFieldValidError::WeakLogin => true,
                _ => false,
            },
            RegistrationFieldValidError::PasswordTooLong => match other {
                RegistrationFieldValidError::PasswordTooLong => true,
                _ => false,
            },
            RegistrationFieldValidError::EqualLoginPassword => match other {
                RegistrationFieldValidError::EqualLoginPassword => true,
                _ => false,
            },
        }
    }
}

impl std::fmt::Display for RegistrationFieldValidError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RegistrationFieldValidError::WeakPassword => write!(f, "Weak password"),
            RegistrationFieldValidError::WeakUsername => write!(f, "Weak username"),
            RegistrationFieldValidError::WeakLogin => write!(f, "Weak login"),
            RegistrationFieldValidError::PasswordTooLong => write!(f, "Password too long"),
            RegistrationFieldValidError::EqualLoginPassword => {
                write!(f, "Equal login and password")
            }
        }
    }
}
