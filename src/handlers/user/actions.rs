use crate::database::{
    self,
    user::{FavouriteError, FollowError, UserDatabase},
};

pub enum FollowResponse {
    Ok,
    Error(FollowError),
}

pub enum FavouriteResponse {
    Ok,
    Error(FavouriteError),
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

pub async fn add_favourite<'a>(
    uuid: &'a str,
    favourite_uuid: &'a str,
    title: &'a str,
    db: database::Conn,
) -> FavouriteResponse {
    match db.add_favourite(uuid, favourite_uuid, title).await {
        database::DatabaseResponse::Ok => FavouriteResponse::Ok,
        database::DatabaseResponse::Err(err) => FavouriteResponse::Error(err),
    }
}

pub async fn remove_favourite<'a>(
    uuid: &'a str,
    favourite_uuid: &'a str,
    db: database::Conn,
) -> FavouriteResponse {
    match db.remove_favourite(uuid, favourite_uuid).await {
        database::DatabaseResponse::Ok => FavouriteResponse::Ok,
        database::DatabaseResponse::Err(err) => FavouriteResponse::Error(err),
    }
}

pub async fn is_favourite<'a>(
    uuid: &'a str,
    title: &'a str,
    db: database::Conn,
) -> Result<bool, FavouriteError> {
    match db.is_favourite(uuid, title).await {
        Ok(is_favourite) => Ok(is_favourite),
        Err(err) => Err(err),
    }
}
