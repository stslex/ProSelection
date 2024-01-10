use crate::database::auth::reg_objects::RegistrationData;

use super::{registration::RegistrationError, AuthValidation, RegistrationFieldValid};

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

impl<'a> AuthValidation for RegistrationData {
    fn validate(&self) -> Result<Self, RegistrationError> {
        if self.password.len() < 8 {
            return Result::Err(RegistrationError::WeakPassword);
        }
        if self.username.len() < 6 {
            return Result::Err(RegistrationError::WeakUsername);
        }
        if self.login.len() < 6 {
            return Result::Err(RegistrationError::WeakLogin);
        }
        if self.password.len() > 16 {
            return Result::Err(RegistrationError::PasswordTooLong);
        }
        if self.password == self.login
            || self.password == self.username
            || self.login == self.username
        {
            return Result::Err(RegistrationError::EqualLoginPassword);
        }
        Result::Ok(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        database::auth::reg_objects::RegistrationData,
        handlers::auth::fields_valid::{AuthValidation, RegistrationError},
    };

    #[test]
    fn test_validate_weak_password() {
        let data = RegistrationData {
            password: "weak".to_owned(),
            username: "username".to_owned(),
            login: "loginCorrect".to_owned(),
        };
        assert_eq!(
            data.validate().unwrap_err(),
            RegistrationError::WeakPassword
        );
    }

    #[test]
    fn test_validate_weak_username() {
        let data = RegistrationData {
            password: "password".to_owned(),
            username: "weak".to_owned(),
            login: "loginCorrect".to_owned(),
        };
        assert_eq!(
            data.validate().unwrap_err(),
            RegistrationError::WeakUsername
        );
    }

    #[test]
    fn test_validate_weak_login() {
        let data = RegistrationData {
            password: "password".to_owned(),
            username: "username".to_owned(),
            login: "weak".to_owned(),
        };
        assert_eq!(data.validate().unwrap_err(), RegistrationError::WeakLogin);
    }

    #[test]
    fn test_validate_password_too_long() {
        let data = RegistrationData {
            password: "veryverylongpassword".to_owned(),
            username: "username".to_owned(),
            login: "loginCorrect".to_owned(),
        };
        assert_eq!(
            data.validate().unwrap_err(),
            RegistrationError::PasswordTooLong
        );
    }

    #[test]
    fn test_validate_equal_login_password() {
        let data = RegistrationData {
            password: "password".to_owned(),
            username: "username".to_owned(),
            login: "password".to_owned(),
        };
        assert_eq!(
            data.validate().unwrap_err(),
            RegistrationError::EqualLoginPassword
        );
    }

    #[test]
    fn test_validate_equal_login_username() {
        let data = RegistrationData {
            password: "password".to_owned(),
            username: "loginCorrect".to_owned(),
            login: "loginCorrect".to_owned(),
        };
        assert_eq!(
            data.validate().unwrap_err(),
            RegistrationError::EqualLoginPassword
        );
    }

    #[test]
    fn test_validate_equal_lpassword_username() {
        let data = RegistrationData {
            password: "password".to_owned(),
            username: "password".to_owned(),
            login: "loginCorrect".to_owned(),
        };
        assert_eq!(
            data.validate().unwrap_err(),
            RegistrationError::EqualLoginPassword
        );
    }

    #[test]
    fn test_validate_valid_data() {
        let data = RegistrationData {
            password: "password".to_owned(),
            username: "username".to_owned(),
            login: "loginCorrect".to_owned(),
        };
        let result = data.validate().unwrap();
        assert_eq!(result.login, data.login);
        assert_eq!(result.username, data.username);
        assert_eq!(result.password, data.password);
    }
}
