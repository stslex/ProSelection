use uuid::Uuid;

use crate::{
    data::database::{
        follow::{objects::FollowEntityCreate, FollowDatabase},
        user::UserDatabase,
    },
    utils::Mapper,
    Conn,
};

use super::{
    objects::{FollowDataError, FollowPagingDataRequest, FollowerDataResponse},
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
    ) -> Result<(), FollowDataError> {
        let followed_user = UserDatabase::get_user(self, followed_uuid)
            .await
            .map_err(|err| {
                eprintln!("Error getting user: {}", err);
                FollowDataError::UserNotFound
            })?;
        let follower_uuid =
            Uuid::parse_str(follower_uuid).map_err(|_| FollowDataError::UuidInvalid)?;

        let record = FollowEntityCreate {
            follower_uuid: follower_uuid.to_owned(),
            followed_uuid: followed_user.id.to_owned(),
            username: followed_user.username,
            avatar_url: followed_user.avatar_url,
        };
        FollowDatabase::follow_user(self, &record).await
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
        request: &'a FollowPagingDataRequest<'a>,
    ) -> Result<Vec<FollowerDataResponse>, FollowDataError> {
        match FollowDatabase::get_user_followers(self, request).await {
            Ok(followers) => Ok(followers.map().await),
            Err(err) => Err(err),
        }
    }
    async fn get_user_following<'a>(
        &self,
        request: &'a FollowPagingDataRequest<'a>,
    ) -> Result<Vec<FollowerDataResponse>, FollowDataError> {
        match FollowDatabase::get_user_following(self, request).await {
            Ok(following) => Ok(following.map().await),
            Err(err) => Err(err),
        }
    }
}
