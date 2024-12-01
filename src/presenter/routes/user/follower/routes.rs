use rocket::serde::json::Json;

use crate::data::repository::follow::objects::FollowDataError;
use crate::data::repository::user::objects::UserSearchError;
use crate::presenter::handlers;

use crate::presenter::handlers::objects::request::PagingUuidRequest;
use crate::presenter::handlers::objects::response::{
    ApiMessageResponse, ApiResponse, ERROR_FOLLOW_CONFLICT, ERROR_FOLLOW_USER_NOT_FOUND,
    ERROR_FOLLOW_UUID_INVALID, ERROR_UNKNOWN, ERROR_USER_NOT_FOUND_BY_UUID,
};
use crate::presenter::handlers::objects::response::{BooleanResponse, PagingResponse};
use crate::presenter::handlers::user::actions::{self, FollowResponse};

use crate::presenter::handlers::user::search::FollowerResponse;
use crate::presenter::routes::auth::validators::AccessToken;
use crate::Conn;

#[get("/followers?<params..>")]
pub async fn get_user_followers<'a>(
    access_token: AccessToken,
    params: PagingUuidRequest<'a>,
    db: Conn,
) -> ApiResponse<'static, Json<PagingResponse<FollowerResponse>>> {
    let request = handlers::user::search::UserPagingSearchRequest {
        request_uuid: &access_token.uuid,
        uuid: params.uuid,
        query: params.query,
        page: params.page,
        page_size: params.page_size,
    };
    match handlers::user::search::get_user_followers(&request, db).await {
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

#[get("/following?<params..>")]
pub async fn get_user_following<'a>(
    access_token: AccessToken,
    params: PagingUuidRequest<'a>,
    db: Conn,
) -> ApiResponse<'static, Json<PagingResponse<FollowerResponse>>> {
    let request = handlers::user::search::UserPagingSearchRequest {
        request_uuid: &access_token.uuid,
        uuid: params.uuid,
        query: params.query,
        page: params.page,
        page_size: params.page_size,
    };
    match handlers::user::search::get_user_following(&request, db).await {
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

#[post("/<uuid>")]
pub async fn post_follow(
    access_token: AccessToken,
    uuid: String,
    db: Conn,
) -> ApiMessageResponse<'static> {
    match actions::follow_user(&access_token.uuid, &uuid, db).await {
        FollowResponse::Ok => ApiMessageResponse::Ok("success"),
        FollowResponse::Error(err) => {
            eprint!("Error: {:?}", err);
            match err {
                FollowDataError::UuidInvalid => ApiMessageResponse::Err(ERROR_FOLLOW_UUID_INVALID),
                FollowDataError::UserNotFound => {
                    ApiMessageResponse::Err(ERROR_FOLLOW_USER_NOT_FOUND)
                }
                FollowDataError::Conflict => ApiMessageResponse::Err(ERROR_FOLLOW_CONFLICT),
                FollowDataError::InternalError => ApiMessageResponse::Err(&ERROR_UNKNOWN),
            }
        }
    }
}

#[delete("/<uuid>")]
pub async fn delete_follow(
    access_token: AccessToken,
    uuid: String,
    db: Conn,
) -> ApiMessageResponse<'static> {
    match actions::un_follow_user(&access_token.uuid, &uuid, db).await {
        FollowResponse::Ok => ApiMessageResponse::Ok("success"),
        FollowResponse::Error(err) => {
            eprint!("Error: {:?}", err);
            match err {
                FollowDataError::UuidInvalid => ApiMessageResponse::Err(ERROR_FOLLOW_UUID_INVALID),
                FollowDataError::UserNotFound => {
                    ApiMessageResponse::Err(ERROR_FOLLOW_USER_NOT_FOUND)
                }
                FollowDataError::Conflict => ApiMessageResponse::Err(ERROR_FOLLOW_CONFLICT),
                FollowDataError::InternalError => ApiMessageResponse::Err(&ERROR_UNKNOWN),
            }
        }
    }
}

#[get("/is_follow/<uuid>")]
pub async fn get_is_following(
    access_token: AccessToken,
    uuid: String,
    db: Conn,
) -> ApiResponse<'static, Json<BooleanResponse>> {
    match actions::is_following(&access_token.uuid, &uuid, db).await {
        Ok(is_following) => ApiResponse::Ok(Json(BooleanResponse {
            result: is_following,
        })),
        Err(err) => {
            eprint!("Error: {:?}", err);
            match err {
                FollowDataError::UuidInvalid => ApiResponse::Err(ERROR_FOLLOW_UUID_INVALID),
                FollowDataError::UserNotFound => ApiResponse::Err(ERROR_FOLLOW_USER_NOT_FOUND),
                FollowDataError::Conflict => ApiResponse::Err(ERROR_FOLLOW_CONFLICT),
                FollowDataError::InternalError => ApiResponse::Err(&ERROR_UNKNOWN),
            }
        }
    }
}
