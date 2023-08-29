use crate::routes::route_objects::ApiResponse;
use crate::routes::route_objects::error_response::ERROR_WRONG_REQUEST;

#[get("/hello/<name>")]
pub fn hello_username(name: String) -> &'static str {
    let formatted_hello = format!("{}{}", "hello ".to_owned(), name);
    Box::leak(formatted_hello.to_string().into_boxed_str())
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello"
}

#[get("/error")]
pub fn error() -> ApiResponse<'static, ()> {
    ApiResponse::Err(ERROR_WRONG_REQUEST)
}
