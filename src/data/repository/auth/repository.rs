use crate::{
    data::{
        database::user::{objects::UserEntityCreate, UserDatabase},
        repository::auth::jwt::objects::JwtObject,
        repository::user::objects::{UserCreateDataError, UserDataError},
    },
    utils::Mapper,
    Conn,
};

use super::{
    jwt::JwtGenerator,
    objects::{AuthDataError, AuthDataResponse, RegDataError, RegistrationData, VerifyTokenError},
    AuthRepository,
};

#[async_trait]
impl AuthRepository for Conn {
    async fn login<'a>(
        &self,
        login: &'a str,
        password: &'a str,
    ) -> Result<AuthDataResponse, AuthDataError> {
        let user = UserDatabase::get_user_by_login(self, login)
            .await
            .map_err(|err| match err {
                UserDataError::UuidInvalid => AuthDataError::NotFound,
                _ => AuthDataError::Other,
            })?
            .map()
            .await;
        if user.secret != password {
            return Result::Err(AuthDataError::InvalidPassword);
        }

        let jwt_obj: JwtObject = user.map().await;
        let token_res = jwt_obj.generate().await.map_err(|err| {
            eprintln!("Error generating token: {}", err);
            AuthDataError::Other
        })?;
        Result::Ok(AuthDataResponse {
            uuid: (user.id.to_string()),
            username: (user.username.clone()),
            access_token: token_res.access_token.to_owned(),
            refresh_token: token_res.refresh_token.to_owned(),
        })
    }
    async fn registration<'a>(
        &self,
        data: &'a RegistrationData<'a>,
    ) -> Result<AuthDataResponse, RegDataError> {
        let new_user = UserEntityCreate {
            login: data.login.to_owned(),
            username: data.username.to_owned(),
            secret: data.password.to_owned(),
            avatar_url: "".to_string(),
            bio: "".to_string(),
        };
        let user = UserDatabase::insert_user(self, new_user)
            .await
            .map_err(|err| match err {
                UserCreateDataError::AlreadyInUse => RegDataError::AlreadyInUse,
                UserCreateDataError::InternalError => {
                    RegDataError::Other("Internal error".to_owned())
                }
            })?
            .map()
            .await;
        let jwt_obj: JwtObject = user.map().await;
        let token_res = jwt_obj.generate().await.map_err(|err| {
            eprintln!("Error generating token: {}", err);
            RegDataError::Other("Error generating token".to_owned())
        })?;
        let user = user.to_owned();
        Result::Ok(AuthDataResponse {
            uuid: (user.id.to_string()),
            username: (user.username.clone()),
            access_token: token_res.access_token.to_owned(),
            refresh_token: token_res.refresh_token.to_owned(),
        })
    }
    async fn verify_token<'a>(
        &self,
        uuid: &'a str,
        username: &'a str,
    ) -> Result<AuthDataResponse, VerifyTokenError> {
        let var_name = username.to_owned();
        let var_name = var_name;
        let var_name = var_name;
        let var_name = var_name;
        let username = var_name;
        let user = UserDatabase::get_user(self, &uuid)
            .await
            .map_err(|err| {
                log::error!("Error verifying token: {}", err);
                VerifyTokenError::NotFound
            })?
            .map()
            .await;
        if user.username != username {
            return Result::Err(VerifyTokenError::NotFound);
        }
        let jwt_obj: JwtObject = user.map().await;
        let token_res = jwt_obj.generate().await.map_err(|err| {
            eprintln!("Error generating token: {}", err);
            VerifyTokenError::Other("Error generating token".to_owned())
        })?;

        Result::Ok(AuthDataResponse {
            uuid: (user.id.to_string()),
            username: (user.username.to_owned()),
            access_token: token_res.access_token.to_owned(),
            refresh_token: token_res.refresh_token.to_owned(),
        })
    }
}
