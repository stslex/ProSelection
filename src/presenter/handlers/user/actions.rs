use crate::data::database::{
    self,
    user::{FollowError, UserDatabase as _},
};

pub enum FollowResponse {
    Ok,
    Error(FollowError),
}

pub async fn follow_user<'a>(
    follower_uuid: &'a str,
    followed_uuid: &'a str,
    db: database::Conn,
) -> FollowResponse {
    match db.follow_user(follower_uuid, followed_uuid).await {
        database::DatabaseResponse::Ok => FollowResponse::Ok,
        database::DatabaseResponse::Err(err) => FollowResponse::Error(err),
    }
}

pub async fn un_follow_user<'a>(
    follower_uuid: &'a str,
    followed_uuid: &'a str,
    db: database::Conn,
) -> FollowResponse {
    match db.un_follow_user(follower_uuid, followed_uuid).await {
        database::DatabaseResponse::Ok => FollowResponse::Ok,
        database::DatabaseResponse::Err(err) => FollowResponse::Error(err),
    }
}

pub async fn is_following<'a>(
    follower_uuid: &'a str,
    followed_uuid: &'a str,
    db: database::Conn,
) -> Result<bool, FollowError> {
    match db.is_following(follower_uuid, followed_uuid).await {
        Ok(is_following) => Ok(is_following),
        Err(err) => Err(err),
    }
}
