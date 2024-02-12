use rocket::futures;
use uuid::Uuid;

use crate::{data::database::follow::objects::FollowerEntity, utils::Mapper};

/// Represents a follower object retrieved from the database.
#[derive(Debug, Clone)]
pub struct FollowerDataResponse {
    pub uuid: Uuid,
    pub follower_uuid: Uuid,
    pub followed_uuid: Uuid,
    pub username: String,
    pub avatar_url: String,
}

#[async_trait]
impl Mapper<FollowerDataResponse> for FollowerEntity {
    async fn map(&self) -> FollowerDataResponse {
        FollowerDataResponse {
            uuid: self.uuid,
            follower_uuid: self.follower_uuid,
            followed_uuid: self.followed_uuid,
            username: self.username.to_owned(),
            avatar_url: self.avatar_url.to_owned(),
        }
    }
}

#[async_trait]
impl Mapper<Vec<FollowerDataResponse>> for Vec<FollowerEntity> {
    async fn map(&self) -> Vec<FollowerDataResponse> {
        futures::future::join_all(self.into_iter().map(|follower| follower.map())).await
    }
}