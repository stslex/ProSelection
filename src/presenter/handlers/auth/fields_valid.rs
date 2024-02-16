use crate::data::repository::auth::objects::RegistrationData;

use super::{registration::RegistrationError, AuthValidation};

impl<'a> AuthValidation for RegistrationData<'a> {
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
        data::repository::auth::objects::RegistrationData,
        presenter::handlers::auth::fields_valid::{AuthValidation, RegistrationError},
    };

    #[test]
    fn test_validate_weak_password() {
        let data = RegistrationData {
            password: "weak",
            username: "username",
            login: "loginCorrect",
        };
        assert_eq!(
            data.validate().unwrap_err(),
            RegistrationError::WeakPassword
        );
    }

    #[test]
    fn test_validate_weak_username() {
        let data = RegistrationData {
            password: "password",
            username: "weak",
            login: "loginCorrect",
        };
        assert_eq!(
            data.validate().unwrap_err(),
            RegistrationError::WeakUsername
        );
    }

    #[test]
    fn test_validate_weak_login() {
        let data = RegistrationData {
            password: "password",
            username: "username",
            login: "weak",
        };
        assert_eq!(data.validate().unwrap_err(), RegistrationError::WeakLogin);
    }

    #[test]
    fn test_validate_password_too_long() {
        let data = RegistrationData {
            password: "veryverylongpassword",
            username: "username",
            login: "loginCorrect",
        };
        assert_eq!(
            data.validate().unwrap_err(),
            RegistrationError::PasswordTooLong
        );
    }

    #[test]
    fn test_validate_equal_login_password() {
        let data = RegistrationData {
            password: "password",
            username: "username",
            login: "password",
        };
        assert_eq!(
            data.validate().unwrap_err(),
            RegistrationError::EqualLoginPassword
        );
    }

    #[test]
    fn test_validate_equal_login_username() {
        let data = RegistrationData {
            password: "password",
            username: "loginCorrect",
            login: "loginCorrect",
        };
        assert_eq!(
            data.validate().unwrap_err(),
            RegistrationError::EqualLoginPassword
        );
    }

    #[test]
    fn test_validate_equal_lpassword_username() {
        let data = RegistrationData {
            password: "password",
            username: "password",
            login: "loginCorrect",
        };
        assert_eq!(
            data.validate().unwrap_err(),
            RegistrationError::EqualLoginPassword
        );
    }

    #[test]
    fn test_validate_valid_data() {
        let data = RegistrationData {
            password: "password",
            username: "username",
            login: "loginCorrect",
        };
        let result = data.validate().unwrap();
        assert_eq!(result.login, data.login);
        assert_eq!(result.username, data.username);
        assert_eq!(result.password, data.password);
    }
}
