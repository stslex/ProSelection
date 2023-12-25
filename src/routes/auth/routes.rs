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
pub async fn login<'a>(
    login_request: Option<Json<LoginRequest<'a>>>,
    _api_key_validator: validators::ApiKey,
    db: database::Conn,
) -> ApiResponse<'static, Json<LoginOk>> {
    match login_request {
        Some(r) => match auth::login::login(r.login, r.password, db).await {
            Ok(outcome) => ApiResponse::Ok(Json(outcome)),
            Err(LoginError::NotFound) => ApiResponse::Err(ERROR_USER_NOT_FOUND),
            Err(LoginError::Other) => ApiResponse::Err(ERROR_UNKNOWN),
        },
        None => ApiResponse::Err(ERROR_WRONG_REQUEST),
    }
}

#[post("/registration", format = "json", data = "<registration_request>")]
pub async fn registration<'a>(
    registration_request: Option<Json<RegistrationRequest<'a>>>,
    _api_key_validator: validators::ApiKey,
    db: database::Conn,
) -> ApiResponse<'static, Json<LoginOk>> {
    match registration_request
        .map(|r| auth::registration::registration(&r.login, &r.username, &r.password, db))
    {
        Some(option) => match option.await {
            Ok(outcome) => ApiResponse::Ok(Json(outcome)),
            Err(RegistrationError::LoginInUse) => ApiResponse::Err(ERROR_ALREADY_REGISTERED),
            Err(RegistrationError::WeakPassword) => ApiResponse::Err(ERROR_WEAK_PASSWORD),
            Err(RegistrationError::WeakLogin) => ApiResponse::Err(ERROR_WEAK_LOGIN),
            Err(RegistrationError::WeakUsername) => ApiResponse::Err(ERROR_WEAK_USERNAME),
            Err(RegistrationError::PasswordTooLong) => ApiResponse::Err(ERROR_PASSWORD_TOO_LONG),
            Err(RegistrationError::EqualLoginPassword) => ApiResponse::Err(ERROR_EQUAL_DATA),
            Err(RegistrationError::Other) => ApiResponse::Err(ERROR_UNKNOWN),
        },

        None => ApiResponse::Err(ERROR_WRONG_REQUEST),
    }
}

#[get("/refresh")]
pub async fn refresh(
    refresh_token: validators::RefreshToken,
    db: database::Conn,
) -> ApiResponse<'static, Json<RefreshOk>> {
    match auth::refresh::refresh(&refresh_token.uuid, &refresh_token.username, db).await {
        Ok(outcome) => ApiResponse::Ok(Json(outcome)),
        Err(auth::refresh::RefreshError::InvalidRefreshToken) => {
            ApiResponse::Err(ERROR_TOKEN_SIGNATURE)
        }
        _ => ApiResponse::Err(ERROR_UNKNOWN),
    }
}
