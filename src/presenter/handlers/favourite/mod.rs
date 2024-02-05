use rocket::serde::json::Json;

use super::objects::response::{ApiMessageResponse, ApiResponse, BooleanResponse};

mod favourite_handler;
pub mod request;
pub mod response;

#[async_trait]
pub trait FavouriteHandler {
    async fn add_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
        title: &'a str,
    ) -> ApiMessageResponse<'static>;

    async fn remove_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> ApiMessageResponse<'static>;

    async fn is_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> ApiResponse<'static, Json<BooleanResponse>>;
}
