use rocket::response::{Responder, Result};
use rocket::Request;

use crate::routes::route_objects::error_response::ErrorResponse;

pub mod error_response;

pub enum ApiResponse<'a, T> {
    Ok(T),
    Err(&'a ErrorResponse<'a>),
}

#[async_trait]
impl<'r, 'o: 'r, T> Responder<'r, 'o> for ApiResponse<'r, T>
where
    T: Responder<'r, 'o>,
{
    async fn respond_to(self, request: &'r Request<'_>) -> Result<'o> {
        match self {
            ApiResponse::Ok(t) => t.respond_to(request),
            ApiResponse::Err(e) => e.respond_to(request),
        }
    }
}

pub enum ApiMesResponse<'a> {
    Ok(&'static str),
    Err(&'a ErrorResponse<'a>),
}

#[async_trait]
impl<'r, 'o: 'r> Responder<'r, 'o> for ApiMesResponse<'r> {
    async fn respond_to(self, request: &'r Request<'_>) -> Result<'o> {
        match self {
            ApiMesResponse::Ok(t) => t.respond_to(request),
            ApiMesResponse::Err(e) => e.respond_to(request),
        }
    }
}
