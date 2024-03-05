use crate::presenter::handlers::favourite::FavouriteHandler;
use crate::presenter::handlers::objects::request::PagingUuidRequest;
use crate::presenter::handlers::user::search::FavouriteResponse;
use rocket::serde::json::Json;

use crate::data::repository::user::objects::UserSearchError;
use crate::presenter::handlers;
use crate::presenter::handlers::favourite::request::{FavouriteAddBody, FavouriteDeleteParams};

use crate::presenter::handlers::objects::response::{
    ApiMessageResponse, ApiResponse, ERROR_UNKNOWN, ERROR_USER_NOT_FOUND_BY_UUID,
};
use crate::presenter::handlers::objects::response::{BooleanResponse, PagingResponse};

use crate::presenter::routes::auth::validators::AccessToken;
use crate::Conn;

#[get("/?<params..>")]
pub async fn get_user_favourites<'a>(
    access_token: AccessToken,
    params: PagingUuidRequest<'a>,
    db: Conn,
) -> ApiResponse<'static, Json<PagingResponse<FavouriteResponse>>> {
    let request = handlers::user::search::UserPagingSearchRequest {
        request_uuid: &access_token.uuid,
        uuid: params.uuid,
        query: params.query,
        page: params.page,
        page_size: params.page_size,
    };
    match handlers::user::search::get_user_favourites(&request, db).await {
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

#[post("/", format = "json", data = "<body>")]
pub async fn post_add_favourite<'a>(
    access_token: AccessToken,
    body: Json<FavouriteAddBody<'a>>,
    db: Conn,
) -> ApiMessageResponse<'static> {
    FavouriteHandler::add_favourite(&db, &access_token.uuid, body.favourite_uuid, body.title).await
}

#[delete("/?<params..>")]
pub async fn delete_remove_favourite<'a>(
    access_token: AccessToken,
    params: FavouriteDeleteParams<'a>,
    db: Conn,
) -> ApiMessageResponse<'static> {
    FavouriteHandler::remove_favourite(&db, &access_token.uuid, params.favourite_uuid).await
}

#[get("/is_favourite?<uuid>")]
pub async fn get_is_favourite(
    access_token: AccessToken,
    uuid: String,
    db: Conn,
) -> ApiResponse<'static, Json<BooleanResponse>> {
    FavouriteHandler::is_favourite(&db, &access_token.uuid, &uuid).await
}
