use super::{
    reg_objects::{RegistrationFieldValid, RegistrationFieldValidError},
    RegistrationData,
};

pub trait AuthValidation {
    fn validate(&self) -> RegistrationFieldValid;
}

impl<'a> AuthValidation for RegistrationData {
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
