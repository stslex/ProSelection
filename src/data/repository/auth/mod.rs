use self::objects::{
    AuthDataError, AuthDataResponse, RegDataError, RegistrationData, VerifyTokenError,
};

pub mod objects;
pub mod repository;
mod tests;

#[async_trait]
pub trait AuthRepository {
    async fn login<'a>(
        &self,
        login: &'a str,
        password: &'a str,
    ) -> Result<AuthDataResponse, AuthDataError>;
    async fn registration<'a>(
        &self,
        data: &'a RegistrationData<'a>,
    ) -> Result<AuthDataResponse, RegDataError>;
    async fn verify_token<'a>(
        &self,
        uuid: &'a str,
        username: &'a str,
    ) -> Result<AuthDataResponse, VerifyTokenError>;
}
