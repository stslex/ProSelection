use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
pub use rocket_sync_db_pools::diesel::Insertable;

use crate::database::auth::{AuthorizationDatabase, AuthorizationOutcome, NewUser};
use crate::database::user::user_objects::user::User;
use crate::database::Conn;
use crate::schema::users;
use crate::utils::jwt_util::objects::JwtMapper;
use crate::utils::jwt_util::JwtGenerator;

use super::reg_objects::RegistrationFieldValid;
use super::reg_validation::AuthValidation;
use super::RegistrationData;
use super::VerifyTokenOutcome;
use crate::database::auth::RegistrationOutcome;

#[async_trait]
impl AuthorizationDatabase for Conn {
    async fn login(&self, login: &str, password: &str) -> AuthorizationOutcome {
        let login = login.to_owned();
        let password = password.to_owned();
        self.run(move |db| {
            match users::table
                .filter(users::login.eq(login.to_lowercase()))
                .get_result::<User>(db)
            {
                Ok(user) => match user.secret == password {
                    true => match user.map().generate() {
                        Ok(token_res) => AuthorizationOutcome::Ok(super::AuthorizationOk {
                            uuid: (user.id.to_string()),
                            username: (user.username.clone()),
                            access_token: token_res.access_token.to_owned(),
                            refresh_token: token_res.refresh_token.to_owned(),
                        }),
                        Err(_) => AuthorizationOutcome::Other,
                    },
                    false => AuthorizationOutcome::NotFound,
                },
                Err(diesel::result::Error::NotFound) => AuthorizationOutcome::NotFound,
                _ => AuthorizationOutcome::Other,
            }
        })
        .await
    }

    async fn registration(&self, data: RegistrationData) -> RegistrationOutcome {
        match data.validate() {
            RegistrationFieldValid::Ok => (),
            RegistrationFieldValid::Error(err) => {
                log::debug!("Registration field validation error: {:?}", err.to_string());
                return RegistrationOutcome::RegistrationFieldValid(err);
            }
        }

        let new_user = NewUser {
            login: data.login.to_owned(),
            username: data.username.to_owned(),
            secret: data.password.to_owned(),
            avatar_url: "".to_string(),
            bio: "".to_string(),
        };

        self.0
            .run(move |db| {
                match diesel::insert_into(users::table)
                    .values(new_user)
                    .get_result::<User>(db)
                {
                    Ok(user) => match user.map().generate() {
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
            })
            .await
    }

    async fn verify_token(&self, uuid: &str, username: &str) -> VerifyTokenOutcome {
        let uuid = uuid.parse::<uuid::Uuid>().unwrap();
        let username = username.to_owned();
        self.0
            .run(move |db| {
                match users::table
                    .filter(users::id.eq(uuid))
                    .filter(users::username.eq(username))
                    .get_result::<User>(db)
                {
                    Ok(user) => match user.map().generate() {
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
            })
            .await
    }
}
