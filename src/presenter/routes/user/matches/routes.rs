use rocket::serde::json::Json;

use crate::presenter::handlers::matches::objects::{
    UserCreateMatchRequest, UserMatchError, UserMatchResponse,
};
use crate::presenter::handlers::matches::MatchesHandler;
use crate::presenter::handlers::objects::response::{
    ERROR_MATCHES_CONFLICT, ERROR_MATCHES_NOT_FOUND, ERROR_MATCHES_UUID_INVALID,
    ERROR_NO_PERMISSION, ERROR_UNKNOWN,
};
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
        Result::Err(e) => ApiResponse::Err(match e {
            UserMatchError::NoPermission => ERROR_NO_PERMISSION,
            UserMatchError::InternalError => ERROR_UNKNOWN,
            UserMatchError::UuidInvalid => ERROR_MATCHES_UUID_INVALID,
            UserMatchError::MatchesNotFound => ERROR_MATCHES_NOT_FOUND,
            UserMatchError::MatchesNotCreated => ERROR_MATCHES_CONFLICT,
        }),
    }
}
