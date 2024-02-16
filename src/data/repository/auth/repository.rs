use crate::{
    data::{
        database::user::{objects::UserEntityCreate, UserDatabase},
        repository::user::objects::{UserCreateDataError, UserDataError},
    },
    utils::{
        jwt_util::{objects::JwtMapper, JwtGenerator},
        Mapper,
    },
    Conn,
};

use super::{
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

        let token_res = JwtMapper::map(&user).generate().await.map_err(|err| {
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
        let token_res = JwtMapper::map(&user).generate().await.map_err(|err| {
            eprintln!("Error generating token: {}", err);
            RegDataError::Other("Error generating token".to_owned())
        })?;
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
        let username = username.to_owned();
        let user = UserDatabase::get_user(self, &uuid)
            .await
            .map_err(|err| match err {
                UserDataError::UuidInvalid => VerifyTokenError::NotFound,
                _ => VerifyTokenError::Other("Error getting user".to_owned()),
            })?
            .map()
            .await;
        if user.username != username {
            return Result::Err(VerifyTokenError::NotFound);
        }
        let token_res = JwtMapper::map(&user).generate().await.map_err(|err| {
            eprintln!("Error generating token: {}", err);
            VerifyTokenError::Other("Error generating token".to_owned())
        })?;
        Result::Ok(AuthDataResponse {
            uuid: (user.id.to_string()),
            username: (user.username.clone()),
            access_token: token_res.access_token.to_owned(),
            refresh_token: token_res.refresh_token.to_owned(),
        })
    }
}
