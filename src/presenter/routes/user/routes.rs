use rocket::serde::json::Json;

use crate::data::repository::user::objects::{UserSearchDataRequest, UserSearchError};
use crate::presenter::handlers;

use crate::presenter::handlers::objects::request::PagingRequest;
use crate::presenter::handlers::objects::response::{
    ApiResponse, ERROR_UNKNOWN, ERROR_USER_NOT_FOUND_BY_UUID, ERROR_USER_UUID_INVALID,
};

use crate::presenter::handlers::user::search::UserSearchResponse;
use crate::presenter::handlers::user::single_user::{UserError, UserResponse};
use crate::presenter::routes::auth::validators::AccessToken;
use crate::Conn;

#[get("/")]
pub async fn get_current_user(
    access_token: AccessToken,
    db: Conn,
) -> ApiResponse<'static, Json<UserResponse>> {
    match handlers::user::single_user::get_user(&access_token.uuid, &access_token.uuid, db).await {
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
    access_token: AccessToken,
    uuid: String,
    db: Conn,
) -> ApiResponse<'static, Json<UserResponse>> {
    match handlers::user::single_user::get_user(&access_token.uuid, &uuid, db).await {
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
    access_token: AccessToken,
    params: PagingRequest<'a>,
    db: Conn,
) -> ApiResponse<'static, Json<UserSearchResponse>> {
    let request = UserSearchDataRequest {
        query: params.query,
        uuid: &access_token.uuid,
        page: params.page,
        page_size: params.page_size,
    };
    match handlers::user::search::search_user(&request, db).await {
        Ok(response) => ApiResponse::Ok(Json(response)),
        Err(err) => {
            eprint!("Error: {:?}", err);
            return match err {
                UserSearchError::UuidInvalid => ApiResponse::Err(ERROR_USER_NOT_FOUND_BY_UUID),
                UserSearchError::InternalError => ApiResponse::Err(&ERROR_UNKNOWN),
            };
        }
    }
}

#[get("/?username&<username>")]
pub async fn get_user_by_username(
    access_token: AccessToken,
    username: String,
    db: Conn,
) -> ApiResponse<'static, Json<UserResponse>> {
    match handlers::user::single_user::get_user_by_username(&access_token.uuid, &username, db).await
    {
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
