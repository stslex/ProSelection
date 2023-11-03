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
use crate::utils::jwt_utils::generate_jwt;

impl AuthorizationDatabase for Conn {
    fn login(&self, login: &str, password: &str) -> AuthorizationOutcome {
        match users::table
            .filter(users::username.eq(login.to_lowercase()))
            .get_result::<User>(&self.0)
        {
            Ok(user) => {
                if user.secret == password {
                    AuthorizationOutcome::Ok(super::AuthorizationOk {
                        uuid: (user.id.to_string()),
                        username: (user.username),
                        token: (generate_jwt()).to_owned(),
                    })
                } else {
                    AuthorizationOutcome::NotFound
                }
            }
            Err(diesel::result::Error::NotFound) => AuthorizationOutcome::NotFound,
            _ => AuthorizationOutcome::Other,
        }
    }

    fn registration(&self, login: &str, password: &str) -> RegistrationOutcome {
        if password.len() < 8 {
            return RegistrationOutcome::WeakPassword;
        }
        let new_user = NewUser {
            username: login,
            secret: password,
        };
        return match diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(&self.0)
        {
            Ok(_) => RegistrationOutcome::Ok,
            Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => RegistrationOutcome::AlreadyInUse,
            _ => RegistrationOutcome::Other,
        };
    }
}
