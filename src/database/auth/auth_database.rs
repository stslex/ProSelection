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
        match users::table
            .filter(users::login.eq(login.to_lowercase()))
            .get_result::<User>(&self.0)
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
    }

    async fn registration(&self, data: RegistrationData) -> RegistrationOutcome {
        match data.validate() {
            RegistrationFieldValid::Ok => (),
            RegistrationFieldValid::Error(err) => {
                log::debug!("Registration field validation error: {:?}", err.to_string());
                return RegistrationOutcome::RegistrationFieldValid(err);
            }
        }
        let login_binding = data.login.to_owned().to_lowercase();
        let new_user = NewUser {
            login: &login_binding,
            username: data.username,
            secret: data.password,
            avatar_url: "",
            bio: "",
        };

        return match diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(&mut &self.0)
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
        };
    }

    async fn verify_token(&self, uuid: &str, username: &str) -> VerifyTokenOutcome {
        let uuid = uuid.parse::<uuid::Uuid>().unwrap();
        match users::table
            .filter(users::id.eq(uuid))
            .filter(users::username.eq(username))
            .get_result::<User>(&self.0)
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
    }
}
