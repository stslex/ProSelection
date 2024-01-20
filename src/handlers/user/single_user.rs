use std::sync::Arc;

use serde::Serialize;

use crate::database::{
    self,
    user::{user_db::GetByUuidError, user_objects::user::User, UserDatabase},
};

pub async fn get_user<'a>(uuid: &'a str, db: database::Conn) -> Result<UserResponse, UserError> {
    let db = Arc::new(db);
    match db.get_user(uuid).await {
        Ok(user) => Ok(map_user_info(&user, db).await),
        Err(err) => match err {
            GetByUuidError::UuidInvalid => Err(UserError::UuidInvalid),
            GetByUuidError::InternalError => Err(UserError::Other),
        },
    }
}

pub async fn get_user_by_username<'a>(
    username: &'a str,
    db: database::Conn,
) -> Result<UserResponse, UserError> {
    let db = Arc::new(db);
    match db.get_user_by_username(username).await {
        Ok(user) => Ok(map_user_info(&user, db).await),
        Err(err) => match err {
            GetByUuidError::UuidInvalid => Err(UserError::UuidInvalid),
            GetByUuidError::InternalError => Err(UserError::Other),
        },
    }
}

pub async fn map_user_info(user: &User, db: Arc<database::Conn>) -> UserResponse {
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
}

#[derive(Serialize)]
pub struct IsFavouriteResponse {
    pub is_favourite: bool,
}

#[derive(Serialize)]
pub struct IsFollowingResponse {
    pub is_following: bool,
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
