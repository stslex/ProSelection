use self::registration::RegistrationError;

mod fields_valid;
pub mod login;
pub mod objects;
pub mod refresh;
pub mod registration;

trait AuthValidation {
    fn validate(&self) -> Result<Self, RegistrationError>
    where
        Self: Sized;
}
