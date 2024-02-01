use rocket::futures;
use serde::Serialize;

use crate::database::{
    self,
    user::{user_objects::user::User, UserDatabase},
};

use std::sync::Arc;

pub async fn search_user<'a>(
    request: &'a UserSearchRequest<'a>,
    db: database::Conn,
) -> Result<UserSearchResponse, UserSearchError> {
    let db = Arc::new(db);

    match db.search_users(request).await {
        Ok(users) => Result::Ok(UserSearchResponse {
            result: futures::future::join_all(users.into_iter().map(|user| {
                let db: Arc<database::Conn> = Arc::clone(&db);
                async move { map_user_search_info(request.uuid, &user, db).await }
            }))
            .await,
        }),

        Err(_) => Err(UserSearchError::Other),
    }
}

async fn map_user_search_info(
    uuid: &str,
    user: &User,
    db: Arc<database::Conn>,
) -> UserSearchResponseResult {
    UserSearchResponseResult {
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
        is_following: match db.is_following(uuid, &user.id.to_string()).await {
            Ok(is_following) => is_following,
            Err(_) => false,
        },
    }
}

pub async fn get_user_favourites<'a>(
    request: &'a UserPagingRequest<'a>,
    db: database::Conn,
) -> Result<UserFavouriteResponse, UserSearchError> {
    let db = Arc::new(db);
    match db.get_user_favourites(request).await {
        Ok(favourites) => Result::Ok(UserFavouriteResponse {
            result: favourites
                .into_iter()
                .map(|favourite| FavouriteResponse {
                    uuid: favourite.favourite_uuid.to_string(),
                    title: favourite.title,
                })
                .collect::<Vec<_>>(),
        }),

        Err(_) => Err(UserSearchError::Other),
    }
}

pub async fn get_user_followers<'a>(
    request: &'a UserPagingRequest<'a>,
    db: database::Conn,
) -> Result<UserFollowerResponse, UserSearchError> {
    let db = Arc::new(db);

    match db.get_user_followers(request).await {
        Ok(users) => Result::Ok(UserFollowerResponse {
            result: futures::future::join_all(users.into_iter().map(|user| {
                let db: Arc<database::Conn> = Arc::clone(&db);
                async move {
                    let followed_uuid = user.followed_uuid.to_string().to_owned();
                    let followed_uuid_clone = followed_uuid.clone(); // Clone the followed_uuid value
                    FollowerResponse {
                        uuid: followed_uuid,
                        username: user.username,
                        avatar_url: user.avatar_url,
                        is_following: match db
                            .is_following(&followed_uuid_clone, request.uuid)
                            .await
                        {
                            // Use the cloned value
                            Ok(is_following) => is_following,
                            Err(_) => false,
                        },
                    }
                }
            }))
            .await,
        }),

        Err(_) => Err(UserSearchError::Other),
    }
}

pub async fn get_user_following<'a>(
    request: &'a UserPagingRequest<'a>,
    db: database::Conn,
) -> Result<UserFollowerResponse, UserSearchError> {
    let db = Arc::new(db);

    match db.get_user_following(request).await {
        Ok(users) => Result::Ok(UserFollowerResponse {
            result: futures::future::join_all(users.into_iter().map(|user| {
                let db: Arc<database::Conn> = Arc::clone(&db);
                async move {
                    let followed_uuid = user.followed_uuid.to_string().to_owned();
                    let followed_uuid_clone = followed_uuid.clone(); // Clone the followed_uuid value
                    FollowerResponse {
                        uuid: followed_uuid,
                        username: user.username,
                        avatar_url: user.avatar_url,
                        is_following: match db
                            .is_following(request.uuid, &followed_uuid_clone)
                            .await
                        {
                            // Use the cloned value
                            Ok(is_following) => is_following,
                            Err(_) => false,
                        },
                    }
                }
            }))
            .await,
        }),

        Err(_) => Err(UserSearchError::Other),
    }
}

pub struct UserSearchRequest<'a> {
    pub query: &'a str,
    pub uuid: &'a str,
    pub page: i64,
    pub page_size: i64,
}

pub struct UserPagingRequest<'a> {
    pub uuid: &'a str,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Serialize)]
pub struct UserFollowerResponse {
    pub result: Vec<FollowerResponse>,
}

#[derive(Serialize)]
pub struct FollowerResponse {
    pub uuid: String,
    pub username: String,
    pub avatar_url: String,
    pub is_following: bool,
}

#[derive(Serialize)]
pub struct UserFavouriteResponse {
    pub result: Vec<FavouriteResponse>,
}

#[derive(Serialize)]
pub struct FavouriteResponse {
    pub uuid: String,
    pub title: String,
}

#[derive(Serialize)]
pub struct UserSearchResponse {
    pub result: Vec<UserSearchResponseResult>,
}

#[derive(Serialize)]
pub struct UserSearchResponseResult {
    pub uuid: String,
    pub username: String,
    pub avatar_url: String,
    pub bio: String,
    pub followers_count: i64,
    pub following_count: i64,
    pub favourites_count: i64,
    pub is_following: bool,
}

#[derive(Debug)]
pub enum UserSearchError {
    Other,
}
