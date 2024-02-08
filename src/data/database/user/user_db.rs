use super::{
    user_objects::{user::User, UserCommonOutcome},
    UserDatabase,
};
use crate::{
    data::database::{follow::objects::UserSearchError, Conn},
    presenter::handlers::user::search::UserSearchRequest,
    schema::users,
};
use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use uuid::Uuid;

#[async_trait]
impl UserDatabase for Conn {
    async fn get_user_count(&self) -> UserCommonOutcome<String> {
        self.0
            .run(|db| match users::table.get_results::<User>(db) {
                Ok(items) => UserCommonOutcome::Ok(items.len().to_string()),
                Err(_) => UserCommonOutcome::Error,
            })
            .await
    }

    async fn get_user(&self, uuid: &str) -> Result<User, GetByUuidError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(GetByUuidError::UuidInvalid);
            }
        };
        self.0
            .run(
                move |db| match users::table.filter(users::id.eq(uuid)).first::<User>(db) {
                    Ok(user) => Ok(user),
                    Err(err) => {
                        eprintln!("Error getting user: {}", err);
                        Err(GetByUuidError::InternalError)
                    }
                },
            )
            .await
    }

    async fn search_users(
        &self,
        request: &UserSearchRequest,
    ) -> Result<Vec<User>, UserSearchError> {
        let query = request.query.to_owned();
        let uuid = match Uuid::parse_str(request.uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(UserSearchError::UuidInvalid);
            }
        };
        let limit = request.page_size;
        let offset = request.page * request.page_size;
        self.0
            .run(move |db| {
                let users: Vec<User> = users::table
                    .filter(users::username.ilike(format!("%{}%", query)))
                    .filter(users::id.ne(uuid))
                    .limit(limit)
                    .offset(offset)
                    .get_results::<User>(db)
                    .map_err(|err| {
                        eprintln!("Error getting users: {}", err);
                        UserSearchError::InternalError
                    })?;
                Ok(users)
            })
            .await
    }

    async fn get_user_by_username(&self, username: &str) -> Result<User, GetByUuidError> {
        let username = username.to_owned();
        self.0
            .run(move |db| {
                match users::table
                    .filter(users::username.eq(username))
                    .first::<User>(db)
                {
                    Ok(user) => Ok(user),
                    Err(err) => {
                        eprintln!("Error getting user: {}", err);
                        Err(GetByUuidError::InternalError)
                    }
                }
            })
            .await
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
