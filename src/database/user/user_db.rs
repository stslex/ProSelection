use super::{
    user_objects::{user::User, UserCommonOutcome},
    UserDatabase,
};
use crate::{
    database::Conn,
    schema::{favourite, users},
};
use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
pub use rocket_contrib::databases::diesel::Insertable;
use uuid::Uuid;

impl UserDatabase for Conn {
    fn get_user_count(&self) -> UserCommonOutcome<String> {
        match users::table.get_results::<User>(&self.0) {
            Ok(items) => UserCommonOutcome::Ok(items.len().to_string()),
            Err(_) => UserCommonOutcome::Error,
        }
    }

    fn get_user(&self, uuid: &str) -> Result<User, GetByUuidError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(GetByUuidError::UuidInvalid);
            }
        };
        match users::table
            .filter(users::id.eq(uuid))
            .first::<User>(&self.0)
        {
            Ok(user) => Ok(user),
            Err(err) => {
                eprintln!("Error getting user: {}", err);
                Err(GetByUuidError::InternalError)
            }
        }
    }

    fn get_user_by_username(&self, username: &str) -> Result<User, GetByUuidError> {
        match users::table
            .filter(users::username.eq(username))
            .first::<User>(&self.0)
        {
            Ok(user) => Ok(user),
            Err(err) => {
                eprintln!("Error getting user: {}", err);
                Err(GetByUuidError::InternalError)
            }
        }
    }

    fn get_favourites_count(&self, uuid: &str) -> Result<i64, GetByUuidError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(GetByUuidError::UuidInvalid);
            }
        };
        match favourite::table
            .filter(favourite::uuid.eq(uuid))
            .count()
            .get_result::<i64>(&self.0)
        {
            Ok(count) => Ok(count),
            Err(err) => {
                eprintln!("Error getting user: {}", err);
                Err(GetByUuidError::InternalError)
            }
        }
    }

    fn get_followers_count(&self, uuid: &str) -> Result<i64, GetByUuidError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(GetByUuidError::UuidInvalid);
            }
        };
        match favourite::table
            .filter(favourite::uuid.eq(uuid))
            .count()
            .get_result::<i64>(&self.0)
        {
            Ok(count) => Ok(count),
            Err(err) => {
                eprintln!("Error getting user: {}", err);
                Err(GetByUuidError::InternalError)
            }
        }
    }

    fn get_following_count(&self, uuid: &str) -> Result<i64, GetByUuidError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(GetByUuidError::UuidInvalid);
            }
        };
        match favourite::table
            .filter(favourite::uuid.eq(uuid))
            .count()
            .get_result::<i64>(&self.0)
        {
            Ok(count) => Ok(count),
            Err(err) => {
                eprintln!("Error getting user: {}", err);
                Err(GetByUuidError::InternalError)
            }
        }
    }
}

#[derive(Debug)]
pub enum GetByUuidError {
    UuidInvalid,
    InternalError,
}

impl std::fmt::Display for GetByUuidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetByUuidError::UuidInvalid => write!(f, "UuidInvalid"),
            GetByUuidError::InternalError => write!(f, "InternalError"),
        }
    }
}
