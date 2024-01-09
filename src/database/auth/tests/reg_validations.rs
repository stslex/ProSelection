#[cfg(test)]
mod tests {
    use crate::database::auth::{
        reg_objects::{RegistrationData, RegistrationFieldValid, RegistrationFieldValidError},
        reg_validation::AuthValidation,
    };

    #[test]
    fn test_validate_weak_password() {
        let data = RegistrationData {
            password: "weak".to_owned(),
            username: "username".to_owned(),
            login: "loginCorrect".to_owned(),
        };
        assert_eq!(
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::WeakPassword)
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
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::WeakUsername)
        );
    }

    #[test]
    fn test_validate_weak_login() {
        let data = RegistrationData {
            password: "password".to_owned(),
            username: "username".to_owned(),
            login: "weak".to_owned(),
        };
        assert_eq!(
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::WeakLogin)
        );
    }

    #[test]
    fn test_validate_password_too_long() {
        let data = RegistrationData {
            password: "veryverylongpassword".to_owned(),
            username: "username".to_owned(),
            login: "loginCorrect".to_owned(),
        };
        assert_eq!(
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::PasswordTooLong)
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
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::EqualLoginPassword)
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
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::EqualLoginPassword)
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
            data.validate(),
            RegistrationFieldValid::Error(RegistrationFieldValidError::EqualLoginPassword)
        );
    }

    #[test]
    fn test_validate_valid_data() {
        let data = RegistrationData {
            password: "password".to_owned(),
            username: "username".to_owned(),
            login: "loginCorrect".to_owned(),
        };
        let result = data.validate();
        assert_eq!(result, RegistrationFieldValid::Ok);
    }
}
