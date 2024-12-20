use self::objects::{FollowDataError, FollowerDataResponse};

use super::objects::{PagingDomainRequest, PagingDomainResponse};

pub mod objects;
mod repository;
mod tests;

#[async_trait]
pub trait FollowRepository {
    async fn get_followers_count<'a>(&self, uuid: &'a str) -> Result<i64, FollowDataError>;
    async fn get_following_count<'a>(&self, uuid: &'a str) -> Result<i64, FollowDataError>;
    async fn follow_user<'a>(
        &self,
        follower_uuid: &'a str,
        followed_uuid: &'a str,
    ) -> Result<FollowerDataResponse, FollowDataError>;
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
    ) -> Result<PagingDomainResponse<FollowerDataResponse>, FollowDataError>;
    async fn get_user_following<'a>(
        &self,
        request: &'a PagingDomainRequest<'a>,
    ) -> Result<PagingDomainResponse<FollowerDataResponse>, FollowDataError>;
}
