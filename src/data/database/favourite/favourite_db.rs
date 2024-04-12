use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    data::{
        database::utils::paging::{correct_page_number, parce_uuid},
        repository::objects::{PagingDomainRequest, PagingDomainResponse},
    },
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
        let uuid = parce_uuid(uuid).map_err(|_| FavouriteDbError::UuidInvalid)?;
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
    ) -> Result<FavouriteEntityResponse, super::FavouriteDbError> {
        let is_existing = self.is_favourite(uuid, favourite_uuid).await;

        if is_existing.unwrap_or(false) {
            return Result::Err(super::FavouriteDbError::Conflict);
        }

        let uuid = parce_uuid(uuid).map_err(|_| FavouriteDbError::UuidInvalid)?;
        let favourite_uuid_property =
            parce_uuid(favourite_uuid).map_err(|_| FavouriteDbError::UuidInvalid)?;

        let favourite = FavouriteEntity {
            user_uuid: uuid,
            favourite_uuid: favourite_uuid_property,
            title: title.to_owned(),
        };

        match self
            .0
            .run(move |db| {
                diesel::insert_into(favourite::table)
                    .values(favourite)
                    .get_result(db)
            })
            .await
        {
            Ok(favourite) => {
                log::info!("Added favourite: {:?}", favourite);
                Result::Ok(favourite)
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
        let uuid = parce_uuid(uuid).map_err(|_| FavouriteDbError::UuidInvalid)?;
        let favourite_uuid =
            parce_uuid(favourite_uuid).map_err(|_| FavouriteDbError::UuidInvalid)?;

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
        let uuid = parce_uuid(uuid).map_err(|_| FavouriteDbError::UuidInvalid)?;
        let favourite_uuid =
            parce_uuid(favourite_uuid).map_err(|_| FavouriteDbError::UserNotFound)?;
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
        let request_uuid =
            parce_uuid(request.user_uuid).map_err(|_| FavouriteDbError::UuidInvalid)?;
        let page_number = correct_page_number(request.page);
        let limit = request.page_size;
        let offset = page_number * request.page_size;

        self.0
            .run(move |db| {
                let query_request = favourite::table
                    .filter(favourite::user_uuid.eq(request_uuid))
                    .filter(favourite::title.ilike(format!("%{}%", query)));

                let result_request = query_request.to_owned().limit(limit).offset(offset);
                let results: Vec<FavouriteEntityResponse> = result_request
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

                let result_count = i64::try_from(results.len()).map_err(|err| {
                    eprintln!("Error converting result count: {}", err);
                    FavouriteDbError::InternalError
                })?;

                Ok(PagingDomainResponse {
                    total: total_result,
                    result: results,
                    page: page_number + 1,
                    page_size: request.page_size,
                    has_more: offset + result_count < total_result,
                })
            })
            .await
    }
}
