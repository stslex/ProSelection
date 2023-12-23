use rocket::serde::json::Json;

use crate::database;
use crate::handlers::auth;
use crate::handlers::auth::objects::{LoginError, LoginOk};
use crate::handlers::auth::refresh::RefreshOk;
use crate::handlers::auth::registration::RegistrationError;
use crate::routes::auth::validators;
use crate::routes::route_objects::error_response::{
    ERROR_ALREADY_REGISTERED, ERROR_EQUAL_DATA, ERROR_PASSWORD_TOO_LONG, ERROR_TOKEN_SIGNATURE,
    ERROR_UNKNOWN, ERROR_USER_NOT_FOUND, ERROR_WEAK_LOGIN, ERROR_WEAK_PASSWORD,
    ERROR_WEAK_USERNAME, ERROR_WRONG_REQUEST,
};
use crate::routes::route_objects::ApiResponse;

use super::auth_objects::login_request::LoginRequest;
use super::auth_objects::registration_request::RegistrationRequest;

#[post("/login", format = "json", data = "<login_request>")]
pub async fn login(
    login_request: Option<Json<LoginRequest>>,
    _api_key_validator: validators::ApiKey,
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
pub async fn registration(
    registration_request: Option<Json<RegistrationRequest>>,
    _api_key_validator: validators::ApiKey,
    db: database::Conn,
) -> ApiResponse<'static, Json<LoginOk>> {
    match registration_request
        .map(|r| auth::registration::registration(&r.login, &r.username, &r.password, db))
    {
        Some(Ok(outcome)) => ApiResponse::Ok(Json(outcome)),
        Some(Err(RegistrationError::LoginInUse)) => ApiResponse::Err(ERROR_ALREADY_REGISTERED),
        Some(Err(RegistrationError::WeakPassword)) => ApiResponse::Err(ERROR_WEAK_PASSWORD),
        Some(Err(RegistrationError::WeakUsername)) => ApiResponse::Err(ERROR_WEAK_USERNAME),
        Some(Err(RegistrationError::WeakLogin)) => ApiResponse::Err(ERROR_WEAK_LOGIN),
        Some(Err(RegistrationError::PasswordTooLong)) => ApiResponse::Err(ERROR_PASSWORD_TOO_LONG),
        Some(Err(RegistrationError::EqualLoginPassword)) => ApiResponse::Err(ERROR_EQUAL_DATA),
        Some(Err(RegistrationError::Other)) => ApiResponse::Err(ERROR_UNKNOWN),
        None => ApiResponse::Err(ERROR_WRONG_REQUEST),
    }
}

#[get("/refresh")]
pub async fn refresh(
    refresh_token: validators::RefreshToken,
    db: database::Conn,
) -> ApiResponse<'static, Json<RefreshOk>> {
    match auth::refresh::refresh(&refresh_token.uuid, &refresh_token.username, db) {
        Ok(outcome) => ApiResponse::Ok(Json(outcome)),
        Err(auth::refresh::RefreshError::InvalidRefreshToken) => {
            ApiResponse::Err(ERROR_TOKEN_SIGNATURE)
        }
        _ => ApiResponse::Err(ERROR_UNKNOWN),
    }
}
