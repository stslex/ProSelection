use super::{
    objects::{UserDataError, UserEntity},
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

    async fn search_users<'a>(
        &self,
        request: &'a UserSearchRequest<'a>,
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
}
