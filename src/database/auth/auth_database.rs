use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
pub use rocket_contrib::databases::diesel::Insertable;

use crate::database::auth::{
    AuthorizationDatabase, AuthorizationOutcome, NewUser, RegistrationOutcome,
};
use crate::database::user::user_objects::user::User;
use crate::database::Conn;
use crate::schema::users;
use crate::utils::jwt_utils::JwtMapper;
use crate::utils::jwt_utils::JwtUtil;

impl AuthorizationDatabase for Conn {
    fn login(&self, login: &str, password: &str) -> AuthorizationOutcome {
        match users::table
            .filter(users::login.eq(login.to_lowercase()))
            .get_result::<User>(&self.0)
        {
            Ok(user) => match user.secret == password {
                true => match user.map().generate() {
                    Ok(token_res) => AuthorizationOutcome::Ok(super::AuthorizationOk {
                        uuid: (user.id.to_string()),
                        username: (user.username.clone()),
                        token: token_res,
                    }),
                    Err(_) => AuthorizationOutcome::Other,
                },
                false => AuthorizationOutcome::NotFound,
            },
            Err(diesel::result::Error::NotFound) => AuthorizationOutcome::NotFound,
            _ => AuthorizationOutcome::Other,
        }
    }

    fn registration(&self, login: &str, username: &str, password: &str) -> RegistrationOutcome {
        if password.len() < 8 {
            return RegistrationOutcome::WeakPassword;
        }
        let new_user = NewUser {
            login: login,
            username: username,
            secret: password,
        };
        return match diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(&self.0)
        {
            Ok(user) => match user.map().generate() {
                Ok(token_res) => RegistrationOutcome::Ok(super::AuthorizationOk {
                    uuid: (user.id.to_string()),
                    username: (user.username.clone()),
                    token: token_res,
                }),
                Err(_) => RegistrationOutcome::Other,
            },
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => RegistrationOutcome::AlreadyInUse,
            _ => RegistrationOutcome::Other,
        };
    }
}
