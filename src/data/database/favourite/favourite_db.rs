use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    data::repository::objects::{PagingDomainRequest, PagingDomainResponse},
    schema::favourite,
    Conn,
};

use super::{
    objects::{FavouriteEntity, FavouriteEntityResponse},
    FavouriteDbError, UserFavouritesDatabase,
};

#[async_trait]
impl UserFavouritesDatabase for Conn {
    async fn get_favourites_count<'a>(&self, uuid: &'a str) -> Result<i64, FavouriteDbError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(FavouriteDbError::UuidInvalid);
            }
        };
        self.0
            .run(move |db| {
                match favourite::table
                    .filter(favourite::user_uuid.eq(uuid))
                    .count()
                    .get_result::<i64>(db)
                {
                    Ok(count) => Ok(count),
                    Err(err) => {
                        eprintln!("Error getting user: {}", err);
                        Err(FavouriteDbError::InternalError)
                    }
                }
            })
            .await
    }

    async fn add_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
        title: &'a str,
    ) -> Result<(), super::FavouriteDbError> {
        let is_existing = self.is_favourite(uuid, favourite_uuid).await;

        if is_existing.unwrap_or(false) {
            return Result::Err(super::FavouriteDbError::Conflict);
        }

        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FavouriteDbError::UuidInvalid);
            }
        };
        let favourite_uuid = match Uuid::parse_str(favourite_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FavouriteDbError::UserNotFound);
            }
        };
        let favourite = FavouriteEntity {
            user_uuid: uuid,
            favourite_uuid: favourite_uuid,
            title: title.to_owned(),
        };

        match self
            .0
            .run(move |db| {
                diesel::insert_into(favourite::table)
                    .values(favourite)
                    .execute(db)
            })
            .await
        {
            Ok(res) => {
                log::info!("Added favourite: {:?}", res);
                Result::Ok(())
            }
            Err(err) => {
                eprintln!("Error adding favourite: {}", err);
                Result::Err(super::FavouriteDbError::InternalError)
            }
        }
    }

    async fn remove_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> Result<(), super::FavouriteDbError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FavouriteDbError::UuidInvalid);
            }
        };
        let favourite_uuid = match Uuid::parse_str(favourite_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FavouriteDbError::UserNotFound);
            }
        };

        self.0
            .run(move |db| {
                diesel::delete(
                    favourite::table
                        .filter(favourite::user_uuid.eq(uuid))
                        .filter(favourite::favourite_uuid.eq(favourite_uuid)),
                )
                .execute(db)
                .map(|_| ())
                .map_err(|err| {
                    eprintln!("Error removing favourite: {}", err);
                    super::FavouriteDbError::InternalError
                })
            })
            .await
    }

    async fn is_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> Result<bool, FavouriteDbError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FavouriteDbError::UuidInvalid);
            }
        };
        let favourite_uuid = match Uuid::parse_str(favourite_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FavouriteDbError::UserNotFound);
            }
        };
        self.0
            .run(move |db| {
                favourite::table
                    .filter(favourite::user_uuid.eq(uuid))
                    .filter(favourite::favourite_uuid.eq(favourite_uuid))
                    .first::<FavouriteEntityResponse>(db)
                    .map(|_| true)
                    .or_else(|_| Ok(false))
            })
            .await
    }

    async fn get_user_favourites<'a>(
        &self,
        request: PagingDomainRequest<'a>,
    ) -> Result<PagingDomainResponse<FavouriteEntityResponse>, FavouriteDbError> {
        let query = request.query.to_owned();
        let request_uuid = match Uuid::parse_str(request.user_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(FavouriteDbError::UuidInvalid);
            }
        };
        let page_number = if request.page <= 0 {
            1
        } else {
            request.page - 1
        };
        let limit = request.page_size;
        let offset = page_number * request.page_size;

        self.0
            .run(move |db| {
                let query_request = favourite::table
                    .filter(favourite::user_uuid.eq(request_uuid))
                    .filter(favourite::title.ilike(format!("%{}%", query)));

                let result_request = query_request.to_owned().limit(limit).offset(offset);
                let results = result_request
                    .get_results::<FavouriteEntityResponse>(db)
                    .map_err(|err| {
                        eprintln!("Error getting users: {}", err);
                        FavouriteDbError::InternalError
                    })?;

                let total_request = query_request.to_owned();
                let total_result = total_request.count().get_result(db).map_err(|err| {
                    println!("Error getting total count: {}", err);
                    FavouriteDbError::InternalError
                })?;

                Ok(PagingDomainResponse {
                    total: total_result,
                    result: results,
                    page: page_number,
                    page_size: request.page_size,
                    has_more: offset < total_result,
                })
            })
            .await
    }
}
