use self::objects::{FollowDataError, FollowerDataResponse};

use super::objects::PagingDomainRequest;

pub mod objects;
mod repository;

#[async_trait]
pub trait FollowRepository {
    async fn get_followers_count<'a>(&self, uuid: &'a str) -> Result<i64, FollowDataError>;
    async fn get_following_count<'a>(&self, uuid: &'a str) -> Result<i64, FollowDataError>;
    async fn follow_user<'a>(
        &self,
        follower_uuid: &'a str,
        followed_uuid: &'a str,
    ) -> Result<(), FollowDataError>;
    async fn un_follow_user<'a>(
        &self,
        follower_uuid: &'a str,
        followed_uuid: &'a str,
    ) -> Result<(), FollowDataError>;
    async fn is_following<'a>(
        &self,
        follower_uuid: &'a str,
        followed_uuid: &'a str,
    ) -> Result<bool, FollowDataError>;
    async fn get_user_followers<'a>(
        &self,
        request: &'a PagingDomainRequest<'a>,
    ) -> Result<Vec<FollowerDataResponse>, FollowDataError>;
    async fn get_user_following<'a>(
        &self,
        request: &'a PagingDomainRequest<'a>,
    ) -> Result<Vec<FollowerDataResponse>, FollowDataError>;
}
