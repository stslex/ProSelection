use serde::{Deserialize, Serialize};

use crate::data::repository::matches::objects::{MatchesData, MatchesDataError};

#[derive(Deserialize, FromForm)]
#[allow(dead_code)]
pub struct UserCreateMatchRequest<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub expires_at: u128,
    pub participants_uuid: Vec<&'a str>,
    pub cover_url: &'a str,
}

#[derive(Serialize)]
pub struct UserMatchDetailResponse {
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub creator_uuid: String,
    pub participants_uuid: Vec<String>,
    pub cover_url: String,
    pub expires_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

pub enum UserMatchError {
    NoPermission,
    UuidInvalid,
    MatchesNotFound,
    MatchesNotCreated,
    InternalError,
}

impl Into<UserMatchDetailResponse> for MatchesData {
    fn into(self) -> UserMatchDetailResponse {
        UserMatchDetailResponse {
            uuid: self.uuid.to_string(),
            creator_uuid: self.creator_uuid.to_string(),
            participants_uuid: self
                .participants_uuid
                .iter()
                .map(|id| id.to_string())
                .collect(),
            title: self.title.to_owned(),
            description: self.description.to_owned(),
            cover_url: self.cover_url.to_owned(),
            created_at: self.created_at,
            expires_at: self.expires_at,
            updated_at: self.updated_at,
            status: self.status.into(),
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
            MatchesDataError::NoPermission => UserMatchError::NoPermission,
        }
    }
}
