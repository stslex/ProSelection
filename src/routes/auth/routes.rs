use rocket_contrib::json::Json;

use crate::database;
use crate::handlers::auth;
use crate::handlers::auth::objects::{LoginError, LoginOk};
use crate::handlers::auth::registration::RegistrationError;
use crate::routes::route_objects::error_response::{
    ERROR_ALREADY_REGISTERED, ERROR_UNKNOWN, ERROR_USER_NOT_FOUND, ERROR_WEAK_PASSWORD,
    ERROR_WRONG_REQUEST,
};
use crate::routes::route_objects::ApiResponse;

use super::auth_objects::login_request::LoginRequest;
use super::auth_objects::registration_request::RegistrationRequest;

#[post("/login", format = "json", data = "<login_request>")]
pub fn login(
    login_request: Option<Json<LoginRequest>>,
    db: database::Conn,
) -> ApiResponse<'static, Json<LoginOk>> {
    let result: Option<Result<LoginOk, LoginError>> =
        login_request.map(|r| auth::login::login(r.login, r.password, db));
    match result {
        Some(Ok(outcome)) => ApiResponse::Ok(Json(outcome)),
        Some(Err(LoginError::NotFound)) => ApiResponse::Err(ERROR_USER_NOT_FOUND),
        None => ApiResponse::Err(ERROR_WRONG_REQUEST),
        _ => ApiResponse::Err(ERROR_UNKNOWN),
    }
}

#[post("/registration", format = "json", data = "<registration_request>")]
pub fn registration(
    registration_request: Option<Json<RegistrationRequest>>,
    db: database::Conn,
) -> ApiResponse<'static, Json<LoginOk>> {
    match registration_request
        .map(|r| auth::registration::registration(&r.login, &r.username, &r.password, db))
    {
        Some(Ok(outcome)) => ApiResponse::Ok(Json(outcome)),
        Some(Err(RegistrationError::LoginInUse)) => ApiResponse::Err(ERROR_ALREADY_REGISTERED),
        Some(Err(RegistrationError::WeakPassword)) => ApiResponse::Err(ERROR_WEAK_PASSWORD),
        None => ApiResponse::Err(ERROR_WRONG_REQUEST),
        _ => ApiResponse::Err(ERROR_UNKNOWN),
    }
}
