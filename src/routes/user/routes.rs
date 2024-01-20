use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::database::user::{FavouriteError, FollowError};
use crate::handlers::user::actions::{self, FavouriteResponse, FollowResponse};
use crate::handlers::user::search::UserSearchResponse;
use crate::handlers::user::single_user::{IsFollowingResponse, UserError, UserResponse};
use crate::routes::auth::validators::AccessToken;
use crate::routes::route_objects::error_response::{
    ERROR_FAVOURITE_CONFLICT, ERROR_FAVOURITE_USER_NOT_FOUND, ERROR_FAVOURITE_UUID_INVALID,
    ERROR_FOLLOW_CONFLICT, ERROR_FOLLOW_USER_NOT_FOUND, ERROR_FOLLOW_UUID_INVALID, ERROR_UNKNOWN,
    ERROR_USER_NOT_FOUND_BY_UUID, ERROR_USER_UUID_INVALID,
};
use crate::routes::route_objects::{ApiMesResponse, ApiResponse};
use crate::{database, handlers};

#[get("/count")]
pub async fn get_user_count(
    _access_token: AccessToken,
    db: database::Conn,
) -> ApiResponse<'static, String> {
    match handlers::user::common::count(db).await {
        Ok(count) => ApiResponse::Ok(count),
        Err(_) => ApiResponse::Err(ERROR_UNKNOWN),
    }
}

#[get("/")]
pub async fn get_current_user(
    access_token: AccessToken,
    db: database::Conn,
) -> ApiResponse<'static, Json<UserResponse>> {
    match handlers::user::single_user::get_user(&access_token.uuid, db).await {
        Ok(user) => ApiResponse::Ok(Json(user)),
        Err(err) => {
            eprint!("Error: {:?}", err);
            return match err {
                UserError::UuidInvalid => ApiResponse::Err(ERROR_USER_UUID_INVALID),
                UserError::Other => ApiResponse::Err(ERROR_USER_NOT_FOUND_BY_UUID),
            };
        }
    }
}

#[get("/<uuid>")]
pub async fn get_user(
    _access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiResponse<'static, Json<UserResponse>> {
    match handlers::user::single_user::get_user(&uuid, db).await {
        Ok(user) => ApiResponse::Ok(Json(user)),
        Err(err) => {
            eprint!("Error: {:?}", err);
            return match err {
                UserError::UuidInvalid => ApiResponse::Err(ERROR_USER_UUID_INVALID),
                UserError::Other => ApiResponse::Err(ERROR_USER_NOT_FOUND_BY_UUID),
            };
        }
    }
}

#[get("/search?<params..>")]
pub async fn get_user_search<'a>(
    _access_token: AccessToken,
    params: UserSearchParams<'a>,
    db: database::Conn,
) -> ApiResponse<'static, Json<UserSearchResponse>> {
    // handlers::user::search
    log::info!("query: {}", params.query);
    log::info!("page: {}", params.page);
    log::info!("page_size: {}", params.page_size);
    let test_response = UserSearchResponse { users: Vec::new() };
    ApiResponse::Ok(Json(test_response))
}

#[derive(Deserialize, FromForm)]
pub struct UserSearchParams<'a> {
    query: &'a str,
    page: i64,
    page_size: i64,
}

#[get("/?username&<username>")]
pub async fn get_user_by_username(
    _access_token: AccessToken,
    username: String,
    db: database::Conn,
) -> ApiResponse<'static, Json<UserResponse>> {
    match handlers::user::single_user::get_user_by_username(&username, db).await {
        Ok(user) => ApiResponse::Ok(Json(user)),
        Err(err) => {
            eprint!("Error: {:?}", err);
            return match err {
                UserError::UuidInvalid => ApiResponse::Err(ERROR_USER_UUID_INVALID),
                UserError::Other => ApiResponse::Err(ERROR_USER_NOT_FOUND_BY_UUID),
            };
        }
    }
}

#[post("/<uuid>/follow")]
pub async fn post_follow(
    access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiMesResponse<'static> {
    match actions::follow_user(&access_token.uuid, &uuid, db).await {
        FollowResponse::Ok => ApiMesResponse::Ok("success"),
        FollowResponse::Error(err) => {
            eprint!("Error: {:?}", err);
            match err {
                FollowError::UuidInvalid => ApiMesResponse::Err(ERROR_FOLLOW_UUID_INVALID),
                FollowError::UserNotFound => ApiMesResponse::Err(ERROR_FOLLOW_USER_NOT_FOUND),
                FollowError::Conflict => ApiMesResponse::Err(ERROR_FOLLOW_CONFLICT),
                FollowError::InternalError => ApiMesResponse::Err(ERROR_UNKNOWN),
            }
        }
    }
}

#[delete("/<uuid>/follow")]
pub async fn delete_follow(
    access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiMesResponse<'static> {
    match actions::un_follow_user(&access_token.uuid, &uuid, db).await {
        FollowResponse::Ok => ApiMesResponse::Ok("success"),
        FollowResponse::Error(err) => {
            eprint!("Error: {:?}", err);
            match err {
                FollowError::UuidInvalid => ApiMesResponse::Err(ERROR_FOLLOW_UUID_INVALID),
                FollowError::UserNotFound => ApiMesResponse::Err(ERROR_FOLLOW_USER_NOT_FOUND),
                FollowError::Conflict => ApiMesResponse::Err(ERROR_FOLLOW_CONFLICT),
                FollowError::InternalError => ApiMesResponse::Err(ERROR_UNKNOWN),
            }
        }
    }
}

#[get("/<uuid>/is_following")]
pub async fn get_is_following(
    access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiResponse<'static, Json<IsFollowingResponse>> {
    match actions::is_following(&access_token.uuid, &uuid, db).await {
        Ok(is_following) => ApiResponse::Ok(Json(IsFollowingResponse { is_following })),
        Err(err) => {
            eprint!("Error: {:?}", err);
            match err {
                FollowError::UuidInvalid => ApiResponse::Err(ERROR_FOLLOW_UUID_INVALID),
                FollowError::UserNotFound => ApiResponse::Err(ERROR_FOLLOW_USER_NOT_FOUND),
                FollowError::Conflict => ApiResponse::Err(ERROR_FOLLOW_CONFLICT),
                FollowError::InternalError => ApiResponse::Err(ERROR_UNKNOWN),
            }
        }
    }
}

#[post("/<uuid>/favourite")]
pub async fn post_add_favourite(
    access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiMesResponse<'static> {
    match actions::add_favourite(&access_token.uuid, &uuid, db).await {
        FavouriteResponse::Ok => ApiMesResponse::Ok("success"),
        FavouriteResponse::Error(err) => {
            eprint!("Error: {:?}", err);
            match err {
                FavouriteError::UuidInvalid => ApiMesResponse::Err(ERROR_FAVOURITE_UUID_INVALID),
                FavouriteError::UserNotFound => ApiMesResponse::Err(ERROR_FAVOURITE_USER_NOT_FOUND),
                FavouriteError::Conflict => ApiMesResponse::Err(ERROR_FAVOURITE_CONFLICT),
                FavouriteError::InternalError => ApiMesResponse::Err(ERROR_UNKNOWN),
            }
        }
    }
}

#[delete("/<uuid>/favourite")]
pub async fn delete_remove_favourite(
    access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiMesResponse<'static> {
    match actions::remove_favourite(&access_token.uuid, &uuid, db).await {
        FavouriteResponse::Ok => ApiMesResponse::Ok("success"),
        FavouriteResponse::Error(err) => {
            eprint!("Error: {:?}", err);
            match err {
                FavouriteError::UuidInvalid => ApiMesResponse::Err(ERROR_FAVOURITE_UUID_INVALID),
                FavouriteError::UserNotFound => ApiMesResponse::Err(ERROR_FAVOURITE_USER_NOT_FOUND),
                FavouriteError::Conflict => ApiMesResponse::Err(ERROR_FAVOURITE_CONFLICT),
                FavouriteError::InternalError => ApiMesResponse::Err(ERROR_UNKNOWN),
            }
        }
    }
}

#[get("/<uuid>/is_favourite")]
pub async fn get_is_favourite(
    access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiResponse<'static, Json<IsFollowingResponse>> {
    match actions::is_favourite(&access_token.uuid, &uuid, db).await {
        Ok(is_favourite) => ApiResponse::Ok(Json(IsFollowingResponse {
            is_following: is_favourite,
        })),
        Err(err) => {
            eprint!("Error: {:?}", err);
            match err {
                FavouriteError::UuidInvalid => ApiResponse::Err(ERROR_FAVOURITE_UUID_INVALID),
                FavouriteError::UserNotFound => ApiResponse::Err(ERROR_FAVOURITE_USER_NOT_FOUND),
                FavouriteError::Conflict => ApiResponse::Err(ERROR_FAVOURITE_CONFLICT),
                FavouriteError::InternalError => ApiResponse::Err(ERROR_UNKNOWN),
            }
        }
    }
}
