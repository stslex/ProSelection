use rocket_contrib::json::Json;

use crate::handlers::user::single_user::{UserError, UserResponse};
use crate::routes::auth::validators::AccessToken;
use crate::routes::route_objects::error_response::{
    ERROR_UNKNOWN, ERROR_USER_NOT_FOUND_BY_UUID, ERROR_USER_UUID_INVALID,
};
use crate::routes::route_objects::ApiResponse;
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

#[get("/<username>")]
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
