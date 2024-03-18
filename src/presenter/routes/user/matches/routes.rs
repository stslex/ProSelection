use rocket::serde::json::Json;

use crate::presenter::handlers::matches::objects::{UserCreateMatchRequest, UserMatchResponse};
use crate::presenter::handlers::matches::MatchesHandler;
use crate::presenter::handlers::objects::request::PagingRequest;
use crate::presenter::handlers::objects::response::PagingResponse;
use crate::{
    presenter::{handlers::objects::response::ApiResponse, routes::auth::validators::AccessToken},
    Conn,
};

#[post("/?<params..>")]
pub async fn create_match<'a>(
    access_token: AccessToken,
    params: UserCreateMatchRequest<'a>,
    db: Conn,
) -> ApiResponse<'static, Json<UserMatchResponse>> {
    let uuid = access_token.uuid;
    match db.create_match(&uuid, params).await {
        Result::Ok(response) => ApiResponse::Ok(Json(response)),
        Result::Err(e) => ApiResponse::Err(e.into()),
    }
}

#[get("/<match_uuid>")]
pub async fn get_match<'a>(
    access_token: AccessToken,
    match_uuid: String,
    db: Conn,
) -> ApiResponse<'static, Json<UserMatchResponse>> {
    let user_uuid = access_token.uuid;
    match db.get_match(&user_uuid, &match_uuid).await {
        Result::Ok(response) => ApiResponse::Ok(Json(response)),
        Result::Err(e) => ApiResponse::Err(e.into()),
    }
}

#[get("/?<params..>")]
pub async fn get_matches<'a>(
    access_token: AccessToken,
    params: PagingRequest<'a>,
    db: Conn,
) -> ApiResponse<'static, Json<PagingResponse<UserMatchResponse>>> {
    let uuid = access_token.uuid;
    match db.get_matches(&uuid, params).await {
        Result::Ok(response) => ApiResponse::Ok(Json(response)),
        Result::Err(e) => ApiResponse::Err(e.into()),
    }
}
