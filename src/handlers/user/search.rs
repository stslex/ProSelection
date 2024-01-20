use serde::Serialize;

use crate::database;

use super::single_user::UserResponse;

pub async fn search_user<'a>(
    request: &'a UserSearchRequest,
    db: database::Conn,
) -> Result<UserSearchResponse, UserError> {
    todo!()
}

pub struct UserSearchRequest {
    pub query: String,
    pub uuid: String,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Serialize)]
pub struct UserSearchResponse {
    pub users: Vec<UserResponse>,
}

enum UserError {
    Other,
}
