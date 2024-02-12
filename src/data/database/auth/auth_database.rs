use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
pub use rocket_sync_db_pools::diesel::Insertable;

use crate::data::database::auth::{
    AuthorizationDatabase, AuthorizationOutcome, NewUser, RegistrationOutcome,
};
use crate::data::database::user::objects::UserEntity;
use crate::data::database::Conn;
use crate::schema::users;
use crate::utils::jwt_util::objects::JwtMapper;
use crate::utils::jwt_util::JwtGenerator;

use super::RegistrationData;
use super::VerifyTokenOutcome;

#[async_trait]
impl AuthorizationDatabase for Conn {
    async fn login<'a>(&self, login: &'a str, password: &'a str) -> AuthorizationOutcome {
        let login = login.to_owned();
        let password = password.to_owned();
        match self
            .run(move |db| {
                users::table
                    .filter(users::login.eq(login.to_lowercase()))
                    .get_result::<UserEntity>(db)
            })
            .await
        {
            Ok(user) => match user.secret == password {
                true => match user.map().generate().await {
                    Ok(token_res) => AuthorizationOutcome::Ok(super::AuthorizationOk {
                        uuid: (user.id.to_string()),
                        username: (user.username.clone()),
                        access_token: token_res.access_token.to_owned(),
                        refresh_token: token_res.refresh_token.to_owned(),
                    }),
                    Err(_) => AuthorizationOutcome::Other,
                },
                false => AuthorizationOutcome::InvalidPassword,
            },
            Err(diesel::result::Error::NotFound) => AuthorizationOutcome::NotFound,
            _ => AuthorizationOutcome::Other,
        }
    }

    async fn registration<'a>(&self, data: &'a RegistrationData<'a>) -> RegistrationOutcome {
        let new_user = NewUser {
            login: data.login.to_owned(),
            username: data.username.to_owned(),
            secret: data.password.to_owned(),
            avatar_url: "".to_string(),
            bio: "".to_string(),
        };

        match self
            .0
            .run(move |db| {
                diesel::insert_into(users::table)
                    .values(new_user)
                    .get_result::<UserEntity>(db)
            })
            .await
        {
            Ok(user) => match user.map().generate().await {
                Ok(token_res) => RegistrationOutcome::Ok(super::AuthorizationOk {
                    uuid: (user.id.to_string()),
                    username: (user.username.clone()),
                    access_token: token_res.access_token.to_owned(),
                    refresh_token: token_res.refresh_token,
                }),
                Err(err) => {
                    eprintln!("Token generation error: {:?}", err.to_string());
                    RegistrationOutcome::Other(err.to_string())
                }
            },
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => RegistrationOutcome::AlreadyInUse,
            Err(err) => {
                eprintln!("Database error: {:?}", err.to_string());
                RegistrationOutcome::Other(err.to_string())
            }
        }
    }

    async fn verify_token<'a>(&self, uuid: &'a str, username: &'a str) -> VerifyTokenOutcome {
        let uuid = uuid.parse::<uuid::Uuid>().unwrap();
        let username = username.to_owned();
        // User{}.map().generate_access()
        match self
            .0
            .run(move |db| {
                users::table
                    .filter(users::id.eq(uuid))
                    .filter(users::username.eq(username))
                    .get_result::<UserEntity>(db)
            })
            .await
        {
            Ok(user) => match user.map().generate().await {
                Ok(token_res) => VerifyTokenOutcome::Ok(super::AuthorizationOk {
                    uuid: (user.id.to_string()),
                    username: (user.username.clone()),
                    access_token: token_res.access_token.to_owned(),
                    refresh_token: token_res.refresh_token,
                }),
                Err(_) => VerifyTokenOutcome::Other,
            },
            Err(diesel::result::Error::NotFound) => VerifyTokenOutcome::NotFound,
            _ => VerifyTokenOutcome::Other,
        }
    }
}
