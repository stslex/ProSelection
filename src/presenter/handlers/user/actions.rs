use crate::data::database::{
    self,
    follow::{objects::FollowDataError, FollowDatabase},
};

pub enum FollowResponse {
    Ok,
    Error(FollowDataError),
}

pub async fn follow_user<'a>(
    follower_uuid: &'a str,
    followed_uuid: &'a str,
    db: database::Conn,
) -> FollowResponse {
    match db.follow_user(follower_uuid, followed_uuid).await {
        Result::Ok(()) => FollowResponse::Ok,
        Result::Err(err) => FollowResponse::Error(err),
    }
}

pub async fn un_follow_user<'a>(
    follower_uuid: &'a str,
    followed_uuid: &'a str,
    db: database::Conn,
) -> FollowResponse {
    match db.un_follow_user(follower_uuid, followed_uuid).await {
        Result::Ok(()) => FollowResponse::Ok,
        Result::Err(err) => FollowResponse::Error(err),
    }
}

pub async fn is_following<'a>(
    follower_uuid: &'a str,
    followed_uuid: &'a str,
    db: database::Conn,
) -> Result<bool, FollowDataError> {
    match db.is_following(follower_uuid, followed_uuid).await {
        Ok(is_following) => Ok(is_following),
        Err(err) => Err(err),
    }
}
