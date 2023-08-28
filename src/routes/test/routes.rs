use crate::routes::route_objects::error_response::ERROR_WRONG_REQUEST;
use crate::routes::route_objects::ApiResponse;

#[get("/hello/<name>")]
pub fn helloUsername(name: String) -> &'static str {
    let formated_hello = format!("{}{}", "hello ".to_owned(), name);
    Box::leak(formated_hello.to_string().into_boxed_str())
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello"
}

#[get("/error")]
pub fn error() -> ApiResponse<'static, ()> {
    ApiResponse::Err(ERROR_WRONG_REQUEST)
}
