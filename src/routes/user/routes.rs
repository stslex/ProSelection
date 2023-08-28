use crate::{database, handlers};
use crate::routes::route_objects::ApiResponse;
use crate::routes::route_objects::error_response::ERROR_UNKNOWN;

#[get("/count")]
pub fn get_user_count(
    db: database::Conn,
) -> ApiResponse<'static, String> {
    let result = handlers::user::common::count(db);
    match result {
        Ok(count) => ApiResponse::Ok(count),
        Err(_) => ApiResponse::Err(ERROR_UNKNOWN)
    }
}
