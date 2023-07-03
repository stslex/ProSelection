use rocket_contrib::json::Json;

use crate::database;
use crate::routes::route_objects::ApiResponse;
use crate::routes::route_objects::error_response::{ERROR_ALREADY_REGISTERED, ERROR_UNKNOWN, ERROR_USER_NOT_FOUND, ERROR_WEAK_PASSWORD, ERROR_WRONG_REQUEST};
use crate::routes::route_objects::login_request::LoginRequest;
use crate::routes::route_objects::registration_request::RegistrationRequest;

#[post("/login", format = "json", data = "<maybe_login_request>")]
pub fn login(
    maybe_login_request: Option<Json<LoginRequest>>,
    db: database::Conn,
) -> ApiResponse<'static, String> {
    // TODO add login
    ApiResponse::Err(ERROR_WRONG_REQUEST)
}

#[post("/registration", format = "json", data = "<maybe_registration_request>")]
pub fn registration(
    maybe_registration_request: Option<Json<RegistrationRequest>>,
    db: database::Conn,
) -> ApiResponse<'static, ()> {
    // TODO add registration
    ApiResponse::Err(ERROR_WRONG_REQUEST)
}