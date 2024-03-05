use std::sync::Arc;

use serde::Serialize;

use crate::{
    data::repository::{
        favourite::FavouriteRepository,
        follow::FollowRepository,
        matches::MatchesRepository,
        user::{
            objects::{UserDataError, UserDataResponse},
            UserRepository,
        },
    },
    Conn,
};

pub async fn get_user<'a>(
    current_user_uuid: &'a str,
    uuid: &'a str,
    db: Conn,
) -> Result<UserResponse, UserError> {
    let db = Arc::new(db);
    match db.get_user(uuid).await {
        Result::Ok(user) => Ok(map_user_info(current_user_uuid, &user, db).await),
        Result::Err(err) => match err {
            UserDataError::UuidInvalid => Err(UserError::UuidInvalid),
            UserDataError::InternalError => Err(UserError::Other),
        },
    }
}

pub async fn get_user_by_username<'a>(
    uuid: &'a str,
    username: &'a str,
    db: Conn,
) -> Result<UserResponse, UserError> {
    let db = Arc::new(db);
    match db.get_user_by_username(username).await {
        Ok(user) => Ok(map_user_info(uuid, &user, db).await),
        Err(err) => match err {
            UserDataError::UuidInvalid => Err(UserError::UuidInvalid),
            UserDataError::InternalError => Err(UserError::Other),
        },
    }
}

pub async fn map_user_info(uuid: &str, user: &UserDataResponse, db: Arc<Conn>) -> UserResponse {
    UserResponse {
        uuid: user.id.to_string(),
        username: user.username.clone(),
        bio: user.bio.clone(),
        avatar_url: user.avatar_url.clone(),
        followers_count: db
            .get_followers_count(&user.id.to_string())
            .await
            .unwrap_or(0),
        following_count: db
            .get_following_count(&user.id.to_string())
            .await
            .unwrap_or(0),
        favourites_count: db
            .get_favourites_count(&user.id.to_string())
            .await
            .unwrap_or(0),
        matches_count: db
            .get_match_count(uuid, &user.id.to_string())
            .await
            .unwrap_or(0),
        is_following: if uuid == user.id.to_string() {
            false
        } else {
            db.is_following(uuid, &user.id.to_string())
                .await
                .unwrap_or(false)
        },
        is_followed: if uuid == user.id.to_string() {
            false
        } else {
            db.is_following(&user.id.to_string(), uuid)
                .await
                .unwrap_or(false)
        },
        is_current_user: uuid == user.id.to_string(),
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    pub uuid: String,
    pub username: String,
    pub avatar_url: String,
    pub bio: String,
    pub followers_count: i64,
    pub following_count: i64,
    pub favourites_count: i64,
    pub matches_count: i64,
    pub is_following: bool,
    pub is_followed: bool,
    pub is_current_user: bool,
}

pub enum UserError {
    UuidInvalid,
    Other,
}

impl std::fmt::Debug for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserError::UuidInvalid => write!(f, "UuidInvalid"),
            UserError::Other => write!(f, "Other"),
        }
    }
}
