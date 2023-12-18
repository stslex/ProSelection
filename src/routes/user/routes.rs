use rocket_contrib::json::Json;

use crate::database::user::{FavouriteError, FollowError};
use crate::handlers::user::actions::{self, FavouriteResponse, FollowResponse};
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
pub fn get_user_count(
    _access_token: AccessToken,
    db: database::Conn,
) -> ApiResponse<'static, String> {
    let result = handlers::user::common::count(db);
    match result {
        Ok(count) => ApiResponse::Ok(count),
        Err(_) => ApiResponse::Err(ERROR_UNKNOWN),
    }
}

#[get("/")]
pub fn get_current_user(
    access_token: AccessToken,
    db: database::Conn,
) -> ApiResponse<'static, Json<UserResponse>> {
    match handlers::user::single_user::get_user(&access_token.uuid, db) {
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
pub fn get_user(
    _access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiResponse<'static, Json<UserResponse>> {
    match handlers::user::single_user::get_user(&uuid, db) {
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

#[get("/?username&<username>")]
pub fn get_user_by_username(
    _access_token: AccessToken,
    username: String,
    db: database::Conn,
) -> ApiResponse<'static, Json<UserResponse>> {
    match handlers::user::single_user::get_user_by_username(&username, db) {
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
pub fn post_follow(
    access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiMesResponse<'static> {
    match actions::follow_user(&access_token.uuid, &uuid, db) {
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
pub fn delete_follow(
    access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiMesResponse<'static> {
    match actions::un_follow_user(&access_token.uuid, &uuid, db) {
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
pub fn get_is_following(
    access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiResponse<'static, Json<IsFollowingResponse>> {
    match actions::is_following(&access_token.uuid, &uuid, db) {
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
pub fn post_add_favourite(
    access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiMesResponse<'static> {
    match actions::add_favourite(&access_token.uuid, &uuid, db) {
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
pub fn delete_remove_favourite(
    access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiMesResponse<'static> {
    match actions::remove_favourite(&access_token.uuid, &uuid, db) {
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
pub fn get_is_favourite(
    access_token: AccessToken,
    uuid: String,
    db: database::Conn,
) -> ApiResponse<'static, Json<IsFollowingResponse>> {
    match actions::is_favourite(&access_token.uuid, &uuid, db) {
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
