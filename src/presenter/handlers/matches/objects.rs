use serde::{Deserialize, Serialize};

use crate::data::repository::matches::objects::{MatchesData, MatchesDataError};

#[derive(Deserialize, FromForm)]
pub struct UserCreateMatchRequest<'a> {
    pub creator_uuid: &'a str,
    pub user_uuid: Vec<&'a str>,
    pub title: &'a str,
    pub url: &'a str,
    pub description: &'a str,
}

#[derive(Serialize)]
pub struct UserMatchResponse {
    pub id: String,
    pub creator_uuid: String,
    pub user_uuid: Vec<String>,
    pub title: String,
    pub url: String,
    pub description: String,
}

pub enum UserMatchError {
    NoPermission,
    UuidInvalid,
    MatchesNotFound,
    MatchesNotCreated,
    InternalError,
}

impl Into<UserMatchResponse> for MatchesData {
    fn into(self) -> UserMatchResponse {
        UserMatchResponse {
            id: self.id.to_string(),
            creator_uuid: self.creator_uuid.to_string(),
            user_uuid: self.user_id.iter().map(|id| id.to_string()).collect(),
            title: self.title.to_owned(),
            url: self.url.to_owned(),
            description: self.description.to_owned(),
        }
    }
}

impl Into<UserMatchError> for MatchesDataError {
    fn into(self) -> UserMatchError {
        match self {
            MatchesDataError::UuidInvalid => UserMatchError::UuidInvalid,
            MatchesDataError::MatchesNotFound => UserMatchError::MatchesNotFound,
            MatchesDataError::MatchesNotCreated => UserMatchError::MatchesNotCreated,
            MatchesDataError::InternalError => UserMatchError::InternalError,
        }
    }
}
