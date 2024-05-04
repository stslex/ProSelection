use rocket::futures;
use uuid::Uuid;

use crate::{
    data::database::matches::objects::{MatchesDbError, MatchesEntity},
    utils::Mapper,
};

pub struct MatchesData {
    pub uuid: Uuid,
    pub creator_uuid: Uuid,
    pub participants_uuid: Vec<Uuid>,
    pub title: String,
    pub description: String,
    pub status: MatchStatus,
    pub cover_url: String,
    pub expires_at: i64,
    pub updated_at: i64,
    pub created_at: i64,
}

pub enum MatchStatus {
    Pending,
    Active,
    Expired,
    Completed,
    Cancelled,
}

impl Into<String> for MatchStatus {
    fn into(self) -> String {
        match self {
            MatchStatus::Pending => "pending".to_string(),
            MatchStatus::Active => "active".to_string(),
            MatchStatus::Expired => "expired".to_string(),
            MatchStatus::Completed => "completed".to_string(),
            MatchStatus::Cancelled => "cancelled".to_string(),
        }
    }
}

impl Into<MatchStatus> for String {
    fn into(self) -> MatchStatus {
        match self.as_str() {
            "pending" => MatchStatus::Pending,
            "active" => MatchStatus::Active,
            "expired" => MatchStatus::Expired,
            "completed" => MatchStatus::Completed,
            "cancelled" => MatchStatus::Cancelled,
            _ => MatchStatus::Pending,
        }
    }
}

pub struct MatchesDataCreate<'a> {
    pub creator_uuid: &'a str,
    pub participants_uuid: Vec<&'a str>,
    pub title: &'a str,
    pub description: &'a str,
    pub cover_url: &'a str,
    pub created_at: i64,
    pub expires_at: i64,
    pub updated_at: i64,
}

impl Into<MatchesDataError> for MatchesDbError {
    fn into(self) -> MatchesDataError {
        match self {
            MatchesDbError::UuidInvalid => MatchesDataError::UuidInvalid,
            MatchesDbError::MatchesNotFound => MatchesDataError::MatchesNotFound,
            MatchesDbError::MatchesNotCreated => MatchesDataError::MatchesNotCreated,
            MatchesDbError::InternalError => MatchesDataError::InternalError,
        }
    }
}

impl Into<MatchesData> for MatchesEntity {
    fn into(self) -> MatchesData {
        MatchesData {
            uuid: self.uuid,
            creator_uuid: self.creator_uuid,
            participants_uuid: self.participants_uuid,
            title: self.title,
            description: self.description,
            cover_url: self.cover_url,
            status: self.status.into(),
            expires_at: self.expires_at,
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

#[async_trait]
impl Mapper<Result<Uuid, MatchesDataError>> for str {
    async fn map(&self) -> Result<Uuid, MatchesDataError> {
        Uuid::parse_str(self).map_err(|_| MatchesDataError::UuidInvalid)
    }
}

#[async_trait]
impl Mapper<Result<Vec<Uuid>, MatchesDataError>> for Vec<&str> {
    async fn map(&self) -> Result<Vec<Uuid>, MatchesDataError> {
        futures::future::join_all(self.into_iter().map(|uuid| uuid.map()))
            .await
            .into_iter()
            .collect()
    }
}

pub enum MatchesDataError {
    UuidInvalid,
    MatchesNotFound,
    InternalError,
    MatchesNotCreated,
    NoPermission,
}
