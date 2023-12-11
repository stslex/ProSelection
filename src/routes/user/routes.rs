use crate::routes::auth::validators::refresh_token::RefreshToken;
use crate::routes::route_objects::error_response::ERROR_UNKNOWN;
use crate::routes::route_objects::ApiResponse;
use crate::{database, handlers};

#[get("/count")]
pub fn get_user_count(
    _refresh_token: RefreshToken,
    db: database::Conn,
) -> ApiResponse<'static, String> {
    let result = handlers::user::common::count(db);
    match result {
        Ok(count) => ApiResponse::Ok(count),
        Err(_) => ApiResponse::Err(ERROR_UNKNOWN),
    }
}
