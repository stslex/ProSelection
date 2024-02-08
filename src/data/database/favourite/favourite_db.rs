use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{data::database::Conn, schema::favourite};

use super::{
    objects::{FavouriteDataSearchRequest, FavouriteEntity, FavouriteEntityResponse},
    FavouriteError, UserFavouritesDatabase,
};

#[async_trait]
impl UserFavouritesDatabase for Conn {
    async fn get_favourites_count<'a>(&self, uuid: &'a str) -> Result<i64, FavouriteError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(FavouriteError::UuidInvalid);
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
                        Err(FavouriteError::InternalError)
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
    ) -> Result<(), super::FavouriteError> {
        let is_existing = self.is_favourite(uuid, favourite_uuid).await;

        if is_existing.unwrap_or(false) {
            return Result::Err(super::FavouriteError::Conflict);
        }

        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FavouriteError::UuidInvalid);
            }
        };
        let favourite_uuid = match Uuid::parse_str(favourite_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FavouriteError::UserNotFound);
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
                Result::Err(super::FavouriteError::InternalError)
            }
        }
    }

    async fn remove_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> Result<(), super::FavouriteError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FavouriteError::UuidInvalid);
            }
        };
        let favourite_uuid = match Uuid::parse_str(favourite_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FavouriteError::UserNotFound);
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
                    super::FavouriteError::InternalError
                })
            })
            .await
    }

    async fn is_favourite<'a>(
        &self,
        uuid: &'a str,
        favourite_uuid: &'a str,
    ) -> Result<bool, FavouriteError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FavouriteError::UuidInvalid);
            }
        };
        let favourite_uuid = match Uuid::parse_str(favourite_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FavouriteError::UserNotFound);
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
        request: &FavouriteDataSearchRequest<'a>,
    ) -> Result<Vec<FavouriteEntityResponse>, FavouriteError> {
        let request_uuid = match Uuid::parse_str(request.uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(FavouriteError::UuidInvalid);
            }
        };
        let query = request.query.to_owned().to_lowercase();
        let page = if request.page <= 0 {
            1
        } else {
            request.page - 1
        };
        let limit = request.page_size;
        let offset = page * request.page_size;

        self.0
            .run(move |db| {
                let users: Vec<FavouriteEntityResponse> = favourite::table
                    .filter(favourite::user_uuid.eq(request_uuid))
                    .filter(favourite::title.ilike(format!("%{}%", query)))
                    .limit(limit)
                    .offset(offset)
                    .get_results::<FavouriteEntityResponse>(db)
                    .map_err(|err| {
                        eprintln!("Error getting users: {}", err);
                        FavouriteError::InternalError
                    })?;
                Ok(users)
            })
            .await
    }
}
