use crate::{
    data::{
        database::{
            follow::{objects::FollowEntityCreate, FollowDatabase},
            user::UserDatabase,
        },
        repository::objects::PagingDomainRequest,
    },
    utils::Mapper,
    Conn,
};

use super::{
    objects::{FollowDataError, FollowerDataResponse},
    FollowRepository,
};

#[async_trait]
impl FollowRepository for Conn {
    async fn get_followers_count<'a>(&self, uuid: &'a str) -> Result<i64, FollowDataError> {
        FollowDatabase::get_followers_count(self, uuid).await
    }
    async fn get_following_count<'a>(&self, uuid: &'a str) -> Result<i64, FollowDataError> {
        FollowDatabase::get_following_count(self, uuid).await
    }
    async fn follow_user<'a>(
        &self,
        follower_uuid: &'a str,
        followed_uuid: &'a str,
    ) -> Result<FollowerDataResponse, FollowDataError> {
        let followed_user = UserDatabase::get_user(self, followed_uuid)
            .await
            .map_err(|err| {
                eprintln!("Error getting user: {}", err);
                FollowDataError::UserNotFound
            })?;
        let user = UserDatabase::get_user(self, follower_uuid)
            .await
            .map_err(|err| {
                eprintln!("Error getting user: {}", err);
                FollowDataError::UserNotFound
            })?;

        let record = FollowEntityCreate {
            follower_uuid: user.id.to_owned(),
            followed_uuid: followed_user.id.to_owned(),
            followed_username: followed_user.username,
            follower_username: user.username,
            followed_avatar_url: followed_user.avatar_url,
            follower_avatar_url: user.avatar_url,
        };
        match FollowDatabase::follow_user(self, &record).await {
            Ok(follow) => Ok(follow.map().await),
            Err(err) => Err(err),
        }
    }
    async fn un_follow_user<'a>(
        &self,
        follower_uuid: &'a str,
        followed_uuid: &'a str,
    ) -> Result<(), FollowDataError> {
        FollowDatabase::un_follow_user(self, follower_uuid, followed_uuid).await
    }
    async fn is_following<'a>(
        &self,
        follower_uuid: &'a str,
        followed_uuid: &'a str,
    ) -> Result<bool, FollowDataError> {
        FollowDatabase::is_following(self, follower_uuid, followed_uuid).await
    }
    async fn get_user_followers<'a>(
        &self,
        request: &'a PagingDomainRequest<'a>,
    ) -> Result<Vec<FollowerDataResponse>, FollowDataError> {
        match FollowDatabase::get_user_followers(self, request).await {
            Ok(followers) => Ok(followers.map().await),
            Err(err) => Err(err),
        }
    }
    async fn get_user_following<'a>(
        &self,
        request: &'a PagingDomainRequest<'a>,
    ) -> Result<Vec<FollowerDataResponse>, FollowDataError> {
        match FollowDatabase::get_user_following(self, request).await {
            Ok(following) => Ok(following.map().await),
            Err(err) => Err(err),
        }
    }
}
