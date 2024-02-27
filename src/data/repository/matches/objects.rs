use rocket::futures;
use uuid::Uuid;

use crate::{
    data::database::matches::objects::{MatchesDbError, MatchesEntity, MatchesEntityCreate},
    utils::Mapper,
};

pub struct MatchesData {
    pub id: Uuid,
    pub creator_uuid: Uuid,
    pub user_id: Vec<Uuid>,
    pub title: String,
    pub url: String,
    pub description: String,
}

pub struct MatchesDataCreate<'a> {
    pub creator_uuid: &'a str,
    pub user_uuid: Vec<&'a str>,
    pub title: &'a str,
    pub url: &'a str,
    pub description: &'a str,
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
            id: self.id,
            creator_uuid: self.creator_uuid,
            user_id: self.user_id,
            title: self.title,
            url: self.url,
            description: self.description,
        }
    }
}

#[async_trait]
impl<'a> Mapper<Result<MatchesEntityCreate, MatchesDataError>> for MatchesDataCreate<'a> {
    async fn map(&self) -> Result<MatchesEntityCreate, MatchesDataError> {
        Ok(MatchesEntityCreate {
            creator_uuid: self.creator_uuid.map().await?,
            user_uuid: self.user_uuid.map().await?,
            title: self.title.to_owned(),
            url: self.url.to_owned(),
            description: self.description.to_owned(),
        })
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

pub struct MatchesDataRequest<'a> {
    pub user_uuid: &'a str,
    pub query: &'a str,
    pub page: i64,
    pub page_size: i64,
}

pub enum MatchesDataError {
    UuidInvalid,
    MatchesNotFound,
    InternalError,
    MatchesNotCreated,
}
