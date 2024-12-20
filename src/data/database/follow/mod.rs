use uuid::Uuid;

use crate::data::repository::{
    follow::objects::FollowDataError,
    objects::{PagingDomainRequest, PagingDomainResponse},
};

use self::objects::{FollowEntityCreate, FollowerEntity};

mod follow_db;
pub mod objects;
mod tests;

#[async_trait]
pub trait FollowDatabase {
    async fn get_followers_count<'a>(&self, uuid: &'a str) -> Result<i64, FollowDataError>;
    async fn get_following_count<'a>(&self, uuid: &'a str) -> Result<i64, FollowDataError>;
    async fn follow_user<'a>(
        &self,
        record: &'a FollowEntityCreate,
    ) -> Result<FollowerEntity, FollowDataError>;
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
    async fn is_following_uuid<'a>(
        &self,
        follower_uuid: &'a Uuid,
        followed_uuid: &'a Uuid,
    ) -> Result<bool, FollowDataError>;
    async fn get_user_followers<'a>(
        &self,
        request: &'a PagingDomainRequest<'a>,
    ) -> Result<PagingDomainResponse<FollowerEntity>, FollowDataError>;
    async fn get_user_following<'a>(
        &self,
        request: &'a PagingDomainRequest<'a>,
    ) -> Result<PagingDomainResponse<FollowerEntity>, FollowDataError>;
}
