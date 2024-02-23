use rocket::futures;
use uuid::Uuid;

use crate::{data::database::follow::objects::FollowerEntity, utils::Mapper};

/// Represents a follower object retrieved from the database.
#[derive(Debug, Clone)]
pub struct FollowerDataResponse {
    pub uuid: Uuid,
    pub follower_uuid: Uuid,
    pub followed_uuid: Uuid,
    pub followed_username: String,
    pub follower_username: String,
    pub followed_avatar_url: String,
    pub follower_avatar_url: String,
}

#[async_trait]
impl Mapper<FollowerDataResponse> for FollowerEntity {
    async fn map(&self) -> FollowerDataResponse {
        FollowerDataResponse {
            uuid: self.uuid,
            follower_uuid: self.follower_uuid,
            followed_uuid: self.followed_uuid,
            followed_username: self.followed_username.to_owned(),
            follower_username: self.follower_username.to_owned(),
            followed_avatar_url: self.followed_avatar_url.to_owned(),
            follower_avatar_url: self.follower_avatar_url.to_owned(),
        }
    }
}

#[async_trait]
impl Mapper<Vec<FollowerDataResponse>> for Vec<FollowerEntity> {
    async fn map(&self) -> Vec<FollowerDataResponse> {
        futures::future::join_all(self.into_iter().map(|follower| follower.map())).await
    }
}

#[derive(Debug, Clone)]
pub enum FollowDataError {
    UuidInvalid,
    UserNotFound,
    Conflict,
    InternalError,
}

impl std::fmt::Display for FollowDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FollowDataError::UuidInvalid => write!(f, "UuidInvalid"),
            FollowDataError::UserNotFound => write!(f, "UserNotFound"),
            FollowDataError::Conflict => write!(f, "Conflict"),
            FollowDataError::InternalError => write!(f, "InternalError"),
        }
    }
}
