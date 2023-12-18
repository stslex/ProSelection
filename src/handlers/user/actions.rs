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

pub fn follow_user<'a>(
    follower_uuid: &'a str,
    followed_uuid: &'a str,
    db: database::Conn,
) -> FollowResponse {
    match db.follow_user(follower_uuid, followed_uuid) {
        database::DatabaseResponse::Ok => FollowResponse::Ok,
        database::DatabaseResponse::Err(err) => FollowResponse::Error(err),
    }
}

pub fn un_follow_user<'a>(
    follower_uuid: &'a str,
    followed_uuid: &'a str,
    db: database::Conn,
) -> FollowResponse {
    match db.un_follow_user(follower_uuid, followed_uuid) {
        database::DatabaseResponse::Ok => FollowResponse::Ok,
        database::DatabaseResponse::Err(err) => FollowResponse::Error(err),
    }
}

pub fn is_following<'a>(
    follower_uuid: &'a str,
    followed_uuid: &'a str,
    db: database::Conn,
) -> Result<bool, FollowError> {
    match db.is_following(follower_uuid, followed_uuid) {
        Ok(is_following) => Ok(is_following),
        Err(err) => Err(err),
    }
}

pub fn add_favourite<'a>(uuid: &'a str, title: &'a str, db: database::Conn) -> FavouriteResponse {
    match db.add_favourite(uuid, title) {
        database::DatabaseResponse::Ok => FavouriteResponse::Ok,
        database::DatabaseResponse::Err(err) => FavouriteResponse::Error(err),
    }
}

pub fn remove_favourite<'a>(
    uuid: &'a str,
    title: &'a str,
    db: database::Conn,
) -> FavouriteResponse {
    match db.remove_favourite(uuid, title) {
        database::DatabaseResponse::Ok => FavouriteResponse::Ok,
        database::DatabaseResponse::Err(err) => FavouriteResponse::Error(err),
    }
}

pub fn is_favourite<'a>(
    uuid: &'a str,
    title: &'a str,
    db: database::Conn,
) -> Result<bool, FavouriteError> {
    match db.is_favourite(uuid, title) {
        Ok(is_favourite) => Ok(is_favourite),
        Err(err) => Err(err),
    }
}
