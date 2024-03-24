use rocket::futures;
use serde::Serialize;

use super::single_user::{map_user_info, UserResponse};
use crate::{
    data::repository::{
        favourite::{objects::FavouriteDataError, FavouriteRepository},
        follow::{objects::FollowDataError, FollowRepository},
        objects::PagingDomainRequest,
        user::{
            objects::{UserSearchDataRequest, UserSearchError},
            UserRepository,
        },
    },
    presenter::handlers::objects::response::PagingResponse,
    Conn,
};
use std::sync::Arc;

pub async fn search_user<'a>(
    request: &'a UserSearchDataRequest<'a>,
    db: Conn,
) -> Result<UserSearchResponse, UserSearchError> {
    let db = Arc::new(db);

    match db.search_users(request).await {
        Ok(users) => Result::Ok(UserSearchResponse {
            result: futures::future::join_all(users.into_iter().map(|user| {
                let db: Arc<Conn> = Arc::clone(&db);
                async move { map_user_info(request.uuid, &user, db).await }
            }))
            .await,
        }),

        Err(err) => Err(err),
    }
}

pub async fn get_user_favourites<'a>(
    request: &'a UserPagingSearchRequest<'a>,
    db: Conn,
) -> Result<PagingResponse<FavouriteResponse>, UserSearchError> {
    let db = Arc::new(db);
    let request = PagingDomainRequest {
        request_uuid: request.request_uuid,
        user_uuid: request.uuid,
        query: request.query,
        page: request.page,
        page_size: request.page_size,
    };
    match db.get_user_favourites(&request).await {
        Ok(response) => Result::Ok(PagingResponse {
            page: response.page,
            page_size: response.page_size,
            total: response.total,
            has_more: response.has_more,
            result: futures::future::join_all(
                response
                    .result
                    .into_iter()
                    .map(|favourite| {
                        let db: Arc<Conn> = Arc::clone(&db);
                        async move {
                            FavouriteResponse {
                                uuid: favourite.favourite_uuid.to_string(),
                                title: favourite.title,
                                is_favourite: if request.request_uuid
                                    == favourite.user_uuid.to_string()
                                {
                                    true
                                } else {
                                    db.is_favourite(
                                        request.request_uuid,
                                        &favourite.favourite_uuid.to_string(),
                                    )
                                    .await
                                    .unwrap_or(false)
                                },
                            }
                        }
                    })
                    .collect::<Vec<_>>(),
            )
            .await,
        }),

        Err(err) => match err {
            FavouriteDataError::UuidInvalid => Err(UserSearchError::UuidInvalid),
            _ => Err(UserSearchError::InternalError),
        },
    }
}

pub async fn get_user_followers<'a>(
    request: &'a UserPagingSearchRequest<'a>,
    db: Conn,
) -> Result<UserFollowerResponse, UserSearchError> {
    let db = Arc::new(db);

    let follow_request = PagingDomainRequest {
        request_uuid: request.request_uuid,
        user_uuid: request.uuid,
        query: request.query,
        page: request.page,
        page_size: request.page_size,
    };
    match db.get_user_followers(&follow_request).await {
        Ok(users) => Result::Ok(UserFollowerResponse {
            result: futures::future::join_all(users.into_iter().map(|user| {
                let db: Arc<Conn> = Arc::clone(&db);
                async move {
                    let followed_uuid = user.followed_uuid.to_string().to_owned();
                    let followed_uuid_clone = followed_uuid.clone(); // Clone the followed_uuid value
                    FollowerResponse {
                        uuid: followed_uuid,
                        username: user.followed_username,
                        avatar_url: user.followed_avatar_url,
                        is_following: match db
                            .is_following(&followed_uuid_clone, request.request_uuid)
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

        Err(err) => match err {
            FollowDataError::UuidInvalid => Err(UserSearchError::UuidInvalid),
            _ => Err(UserSearchError::InternalError),
        },
    }
}

pub async fn get_user_following<'a>(
    request: &'a UserPagingSearchRequest<'a>,
    db: Conn,
) -> Result<UserFollowerResponse, UserSearchError> {
    let db = Arc::new(db);
    let follow_request = PagingDomainRequest {
        request_uuid: request.request_uuid,
        user_uuid: request.uuid,
        query: request.query,
        page: request.page,
        page_size: request.page_size,
    };
    match db.get_user_following(&follow_request).await {
        Ok(users) => Result::Ok(UserFollowerResponse {
            result: futures::future::join_all(users.into_iter().map(|user| {
                let db: Arc<Conn> = Arc::clone(&db);
                async move {
                    let followed_uuid = user.followed_uuid.to_string().to_owned();
                    let followed_uuid_clone = followed_uuid.clone(); // Clone the followed_uuid value
                    FollowerResponse {
                        uuid: followed_uuid,
                        username: user.followed_username,
                        avatar_url: user.followed_avatar_url,
                        is_following: match db
                            .is_following(request.request_uuid, &followed_uuid_clone)
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

        Err(err) => match err {
            FollowDataError::UuidInvalid => Err(UserSearchError::UuidInvalid),
            _ => Err(UserSearchError::InternalError),
        },
    }
}

pub struct UserPagingSearchRequest<'a> {
    pub request_uuid: &'a str,
    pub uuid: &'a str,
    pub query: &'a str,
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
pub struct FavouriteResponse {
    pub uuid: String,
    pub title: String,
    pub is_favourite: bool,
}

#[derive(Serialize)]
pub struct UserSearchResponse {
    pub result: Vec<UserResponse>,
}
