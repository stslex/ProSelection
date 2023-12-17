use super::{
    user_objects::{user::User, UserCommonOutcome},
    UserDatabase,
};
use crate::{database::Conn, schema::users};
use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
pub use rocket_contrib::databases::diesel::Insertable;
use uuid::Uuid;

impl UserDatabase for Conn {
    fn get_user_count(&self) -> UserCommonOutcome<String> {
        match users::table.get_results::<User>(&self.0) {
            Ok(items) => UserCommonOutcome::Ok(items.len().to_string()),
            Err(_) => UserCommonOutcome::Other,
        }
    }

    fn get_user(&self, uuid: &str) -> Result<User, GetUserError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(GetUserError::UuidInvalid);
            }
        };
        match users::table
            .filter(users::id.eq(uuid))
            .first::<User>(&self.0)
        {
            Ok(user) => Ok(user),
            Err(err) => {
                eprintln!("Error getting user: {}", err);
                Err(GetUserError::InternalError)
            }
        }
    }

    fn get_user_by_username(&self, username: &str) -> Result<User, GetUserError> {
        match users::table
            .filter(users::username.eq(username))
            .first::<User>(&self.0)
        {
            Ok(user) => Ok(user),
            Err(err) => {
                eprintln!("Error getting user: {}", err);
                Err(GetUserError::InternalError)
            }
        }
    }
}

#[derive(Debug)]
pub enum GetUserError {
    UuidInvalid,
    InternalError,
}
