use rocket::http::{ContentType, Status};
use rocket::response::content::Json;
use rocket::response::{Responder, Result};
use rocket::{Request, Response};

#[derive(Copy, Clone)]
pub struct ErrorResponse<'a> {
    cause: &'a str,
    status: Status,
}

impl<'r> Responder<'r> for ErrorResponse<'r> {
    fn respond_to(self, request: &Request) -> Result<'r> {
        if let Ok(response) = Json(json!({"error": self.cause})).respond_to(request) {
            Response::build_from(response)
                .status(self.status)
                .header(ContentType::JSON)
                .ok()
        } else {
            Response::build()
                .status(Status::InternalServerError)
                .header(ContentType::JSON)
                .ok()
        }
    }
}

// common errors
pub const ERROR_UNKNOWN: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "unknown",
    status: Status::InternalServerError,
};
pub const ERROR_WRONG_REQUEST: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "wrong_request",
    status: Status::BadRequest,
};

// login error
pub const ERROR_USER_NOT_FOUND: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "user_not_found",
    status: Status::BadRequest,
};

// registration error
pub const ERROR_WEAK_PASSWORD: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "weak_password",
    status: Status::LengthRequired,
};
pub const ERROR_ALREADY_REGISTERED: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "already_registered",
    status: Status::Conflict,
};
pub const ERROR_WEAK_USERNAME: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "weak_username",
    status: Status::LengthRequired,
};
pub const ERROR_WEAK_LOGIN: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "weak_login",
    status: Status::LengthRequired,
};
pub const ERROR_PASSWORD_TOO_LONG: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "password_too_long",
    status: Status::LengthRequired,
};
pub const ERROR_EQUAL_DATA: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "equal_login_password",
    status: Status::LengthRequired,
};

// token error
pub const ERROR_TOKEN_SIGNATURE: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "token_signature",
    status: Status::Unauthorized,
};

// user error
pub const ERROR_USER_UUID_INVALID: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "uuid_invalid",
    status: Status::BadRequest,
};
pub const ERROR_USER_NOT_FOUND_BY_UUID: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "user_not_found_by_uuid",
    status: Status::BadRequest,
};

// follow error
pub const ERROR_FOLLOW_UUID_INVALID: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "uuid_invalid",
    status: Status::BadRequest,
};
pub const ERROR_FOLLOW_USER_NOT_FOUND: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "user_not_found",
    status: Status::BadRequest,
};
pub const ERROR_FOLLOW_CONFLICT: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "conflict",
    status: Status::Conflict,
};

// favourite error
pub const ERROR_FAVOURITE_UUID_INVALID: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "favourite_uuid_invalid",
    status: Status::BadRequest,
};
pub const ERROR_FAVOURITE_USER_NOT_FOUND: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "favourite_user_not_found",
    status: Status::BadRequest,
};
pub const ERROR_FAVOURITE_CONFLICT: &'static ErrorResponse<'static> = &ErrorResponse {
    cause: "favourite_conflict",
    status: Status::Conflict,
};
