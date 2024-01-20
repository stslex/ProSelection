use rocket::futures;
use serde::Serialize;

use crate::database::{self, user::UserDatabase};

use super::single_user::{map_user_info, UserResponse};
use std::sync::Arc;
pub async fn search_user<'a>(
    request: &'a UserSearchRequest<'a>,
    db: database::Conn,
) -> Result<UserSearchResponse, UserSearchError> {
    let db = Arc::new(db);

    match db.search_users(request).await {
        Ok(users) => Result::Ok(UserSearchResponse {
            users: futures::future::join_all(users.into_iter().map(|user| {
                let db = Arc::clone(&db);
                async move { map_user_info(&user, db).await }
            }))
            .await,
        }),

        Err(_) => Err(UserSearchError::Other),
    }
}

pub struct UserSearchRequest<'a> {
    pub query: &'a str,
    pub uuid: &'a str,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Serialize)]
pub struct UserSearchResponse {
    pub users: Vec<UserResponse>,
}

#[derive(Debug)]
pub enum UserSearchError {
    Other,
}
