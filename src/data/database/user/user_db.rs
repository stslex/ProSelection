use std::collections::HashSet;

use super::{
    objects::{UserEntity, UserEntityCreate},
    UserDatabase,
};
use crate::{
    data::repository::user::objects::{
        UserCreateDataError, UserDataError, UserSearchDataRequest, UserSearchError,
    },
    schema::users,
    Conn,
};
use diesel::RunQueryDsl;
use diesel::{prelude::*, result};
use diesel::{result::DatabaseErrorKind, ExpressionMethods};
use uuid::Uuid;

#[async_trait]
impl UserDatabase for Conn {
    async fn get_user_count(&self) -> Result<String, UserDataError> {
        self.0
            .run(|db| match users::table.get_results::<UserEntity>(db) {
                Ok(items) => Result::Ok(items.len().to_string()),
                Err(_) => Result::Err(UserDataError::InternalError),
            })
            .await
    }

    async fn get_user<'a>(&self, uuid: &'a str) -> Result<UserEntity, UserDataError> {
        let uuid = Uuid::parse_str(uuid).map_err(|_| UserDataError::UuidInvalid)?;
        self.0
            .run(move |db| {
                users::table
                    .filter(users::id.eq(uuid))
                    .first::<UserEntity>(db)
                    .map_err(|_| UserDataError::InternalError)
            })
            .await
    }

    async fn get_user_by_login<'a>(&self, login: &'a str) -> Result<UserEntity, UserDataError> {
        let login = login.to_owned().to_lowercase();
        self.0
            .run(move |db| {
                users::table
                    .filter(users::login.eq(login))
                    .first::<UserEntity>(db)
                    .map_err(|err| {
                        eprintln!("Error getting user: {}", err);
                        UserDataError::InternalError
                    })
            })
            .await
    }

    async fn insert_user<'a>(
        &self,
        user: UserEntityCreate,
    ) -> Result<UserEntity, UserCreateDataError> {
        self.0
            .run(move |db| {
                diesel::insert_into(users::table)
                    .values(user)
                    .get_result::<UserEntity>(db)
            })
            .await
            .map_err(|err| {
                eprintln!("Error inserting user: {}", err);
                match err {
                    result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                        UserCreateDataError::AlreadyInUse
                    }
                    _ => UserCreateDataError::InternalError,
                }
            })
    }

    async fn search_users<'a>(
        &self,
        request: &'a UserSearchDataRequest<'a>,
    ) -> Result<Vec<UserEntity>, UserSearchError> {
        let query = request.query.to_owned();
        let uuid = Uuid::parse_str(request.uuid).map_err(|_| UserSearchError::UuidInvalid)?;
        let limit = request.page_size;
        let offset = request.page * request.page_size;
        self.0
            .run(move |db| {
                users::table
                    .filter(users::username.ilike(format!("%{}%", query)))
                    .filter(users::id.ne(uuid))
                    .limit(limit)
                    .offset(offset)
                    .get_results::<UserEntity>(db)
                    .map_err(|err| {
                        eprintln!("Error getting users: {}", err);
                        UserSearchError::InternalError
                    })
            })
            .await
    }

    async fn get_user_by_username<'a>(
        &self,
        username: &'a str,
    ) -> Result<UserEntity, UserDataError> {
        let username = username.to_owned();
        self.0
            .run(move |db| {
                users::table
                    .filter(users::username.eq(username))
                    .first::<UserEntity>(db)
                    .map_err(|err| {
                        eprintln!("Error getting user: {}", err);
                        UserDataError::InternalError
                    })
            })
            .await
    }

    async fn add_match_to_user<'a>(
        &self,
        user_uuid: &'a str,
        match_uuid: &'a str,
    ) -> Result<(), UserDataError> {
        let match_uuid = Uuid::parse_str(match_uuid).map_err(|_| UserDataError::UuidInvalid)?;
        let user = self.get_user(user_uuid).await.map_err(|e| {
            eprint!("Error adding match: {}", e);
            UserDataError::InternalError
        })?;
        self.0
            .run(move |db| {
                let mut matches = user.matches.to_owned();
                matches.push(match_uuid);

                // Remove duplicates
                let mut set = HashSet::new();
                matches.retain(|e| set.insert(*e));

                diesel::update(users::table.filter(users::id.eq(user.id)))
                    .set(users::matches.eq(matches))
                    .execute(db)
                    .map_err(|err| {
                        eprintln!("Error adding match: {}", err);
                        UserDataError::InternalError
                    })
            })
            .await
            .map_err(|err| {
                eprintln!("Error adding match: {}", err);
                UserDataError::InternalError
            })?;
        Ok(())
    }
}
