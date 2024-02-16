use rocket::serde::json::Json;

use crate::{
    data::repository::favourite::FavouriteRepository,
    presenter::handlers::objects::response::{ApiMessageResponse, ApiResponse, BooleanResponse},
    utils::Mapper,
    Conn,
};

use super::FavouriteHandler;

#[async_trait]
impl FavouriteHandler for Conn {
    async fn add_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
        title: &'a str,
    ) -> ApiMessageResponse<'static> {
        match FavouriteRepository::add_favourite(self, uuid, favourite_uuid, title).await {
            Result::Ok(_) => ApiMessageResponse::Ok("success"),
            Result::Err(err) => ApiMessageResponse::Err(Mapper::map(&err).await),
        }
    }

    async fn remove_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> ApiMessageResponse<'static> {
        match FavouriteRepository::remove_favourite(self, uuid, favourite_uuid).await {
            Result::Ok(_) => ApiMessageResponse::Ok("success"),
            Result::Err(err) => ApiMessageResponse::Err(Mapper::map(&err).await),
        }
    }

    async fn is_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> ApiResponse<'static, Json<BooleanResponse>> {
        match FavouriteRepository::is_favourite(self, uuid, favourite_uuid).await {
            Ok(is_favourite) => ApiResponse::Ok(Json(BooleanResponse {
                result: is_favourite,
            })),
            Err(err) => ApiResponse::Err(Mapper::map(&err).await),
        }
    }
}
