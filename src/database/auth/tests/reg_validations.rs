#[cfg(test)]
mod tests {
    use crate::database::auth::{
        reg_objects::{RegistrationData, RegistrationFieldValid, RegistrationFieldValidError},
        reg_validation::AuthValidation,
    };

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
