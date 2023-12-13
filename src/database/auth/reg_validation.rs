use super::{
    reg_objects::{RegistrationFieldValid, RegistrationFieldValidError},
    RegistrationData,
};

pub trait AuthValidation {
    fn validate(&self) -> RegistrationFieldValid;
}

impl<'a> AuthValidation for RegistrationData<'a> {
    fn validate(&self) -> RegistrationFieldValid {
        if self.password.len() < 8 {
            return RegistrationFieldValid::Error(RegistrationFieldValidError::WeakPassword);
        }
        if self.username.len() < 6 {
            return RegistrationFieldValid::Error(RegistrationFieldValidError::WeakUsername);
        }
        if self.login.len() < 6 {
            return RegistrationFieldValid::Error(RegistrationFieldValidError::WeakLogin);
        }
        if self.password.len() > 16 {
            return RegistrationFieldValid::Error(RegistrationFieldValidError::PasswordTooLong);
        }
        if self.password == self.login
            || self.password == self.username
            || self.login == self.username
        {
            return RegistrationFieldValid::Error(RegistrationFieldValidError::EqualLoginPassword);
        }
        RegistrationFieldValid::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_weak_password() {
        let data = RegistrationData {
            password: "weak",
            username: "username",
            login: "loginCorrect",
        };
        assert_eq!(
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::WeakPassword)
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
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::WeakUsername)
        );
    }

    #[test]
    fn test_validate_weak_login() {
        let data = RegistrationData {
            password: "password",
            username: "username",
            login: "weak",
        };
        assert_eq!(
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::WeakLogin)
        );
    }

    #[test]
    fn test_validate_password_too_long() {
        let data = RegistrationData {
            password: "veryverylongpassword",
            username: "username",
            login: "loginCorrect",
        };
        assert_eq!(
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::PasswordTooLong)
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
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::EqualLoginPassword)
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
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::EqualLoginPassword)
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
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::EqualLoginPassword)
        );
    }

    #[test]
    fn test_validate_valid_data() {
        let data = RegistrationData {
            password: "password",
            username: "username",
            login: "loginCorrect",
        };
        let result = data.validate();
        assert_eq!(result, RegistrationFieldValid::Ok);
    }
}
