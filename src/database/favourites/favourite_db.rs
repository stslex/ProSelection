use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    database::{
        user::user_objects::{Favourite, NewFavourite},
        Conn,
    },
    schema::favourite,
};

use super::{FavouriteError, UserFavouritesDatabase};

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
        let favourite = NewFavourite {
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

    async fn is_favourite(&self, uuid: &str, favourite_uuid: &str) -> Result<bool, FavouriteError> {
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
                    .first::<Favourite>(db)
                    .map(|_| true)
                    .or_else(|_| Ok(false))
            })
            .await
    }
}
