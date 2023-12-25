use super::{
    user_objects::{user::User, Favourite, Follower, NewFollow, UserCommonOutcome},
    FavouriteError, FollowError, UserDatabase,
};
use crate::{
    database::{Conn, DatabaseResponse, OpenError},
    schema::{favourite, follow, users},
};
use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use uuid::Uuid;

#[async_trait]
impl UserDatabase for Conn {
    async fn get_user_count(&self) -> UserCommonOutcome<String> {
        self.0
            .run(|db| match users::table.get_results::<User>(db) {
                Ok(items) => UserCommonOutcome::Ok(items.len().to_string()),
                Err(_) => UserCommonOutcome::Error,
            })
            .await
    }

    async fn get_user(&self, uuid: &str) -> Result<User, GetByUuidError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(GetByUuidError::UuidInvalid);
            }
        };
        self.0
            .run(
                move |db| match users::table.filter(users::id.eq(uuid)).first::<User>(db) {
                    Ok(user) => Ok(user),
                    Err(err) => {
                        eprintln!("Error getting user: {}", err);
                        Err(GetByUuidError::InternalError)
                    }
                },
            )
            .await
    }

    async fn get_user_by_username(&self, username: &str) -> Result<User, GetByUuidError> {
        let username = username.to_owned();
        self.0
            .run(move |db| {
                match users::table
                    .filter(users::username.eq(username))
                    .first::<User>(db)
                {
                    Ok(user) => Ok(user),
                    Err(err) => {
                        eprintln!("Error getting user: {}", err);
                        Err(GetByUuidError::InternalError)
                    }
                }
            })
            .await
    }

    async fn get_favourites_count(&self, uuid: &str) -> Result<i64, GetByUuidError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(GetByUuidError::UuidInvalid);
            }
        };
        self.0
            .run(move |db| {
                match favourite::table
                    .filter(favourite::uuid.eq(uuid))
                    .count()
                    .get_result::<i64>(db)
                {
                    Ok(count) => Ok(count),
                    Err(err) => {
                        eprintln!("Error getting user: {}", err);
                        Err(GetByUuidError::InternalError)
                    }
                }
            })
            .await
    }

    /// Get the number of followers for a user.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The uuid of the user to get the followers count for.
    ///
    async fn get_followers_count(&self, uuid: &str) -> Result<i64, GetByUuidError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(GetByUuidError::UuidInvalid);
            }
        };
        self.0
            .run(move |db| {
                match follow::table
                    .filter(follow::followed_uuid.eq(uuid))
                    .count()
                    .get_result::<i64>(db)
                {
                    Ok(count) => Ok(count),
                    Err(err) => {
                        eprintln!("Error getting user: {}", err);
                        Err(GetByUuidError::InternalError)
                    }
                }
            })
            .await
    }

    /// Get the number of users a user is following.
    ///     
    /// # Arguments
    ///
    /// * `uuid` - The uuid of the user to get the following count for.
    ///
    async fn get_following_count(&self, uuid: &str) -> Result<i64, GetByUuidError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(GetByUuidError::UuidInvalid);
            }
        };
        self.0
            .run(move |db| {
                match follow::table
                    .filter(follow::follower_uuid.eq(uuid))
                    .count()
                    .get_result::<i64>(db)
                {
                    Ok(count) => Ok(count),
                    Err(err) => {
                        eprintln!("Error getting user: {}", err);
                        Err(GetByUuidError::InternalError)
                    }
                }
            })
            .await
    }

    async fn follow_user(
        &self,
        follower_uuid: &str,
        followed_uuid: &str,
    ) -> DatabaseResponse<FollowError> {
        let follower_uuid = match Uuid::parse_str(follower_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return DatabaseResponse::Err(super::FollowError::UuidInvalid);
            }
        };
        let followed_uuid = match Uuid::parse_str(followed_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return DatabaseResponse::Err(super::FollowError::UserNotFound);
            }
        };
        self.0
            .run(move |db| {
                let followed_user = match users::table
                    .filter(users::id.eq(followed_uuid))
                    .first::<User>(db)
                {
                    Ok(user) => user,
                    Err(err) => {
                        eprintln!("Error getting user: {}", err);
                        return DatabaseResponse::Err(super::FollowError::UserNotFound);
                    }
                };
                follow::table
                    .filter(follow::follower_uuid.eq(follower_uuid))
                    .filter(follow::followed_uuid.eq(followed_uuid))
                    .first::<Follower>(db)
                    .map(|_: Follower| DatabaseResponse::Err(super::FollowError::Conflict))
                    .or_else(|_| {
                        let records = NewFollow {
                            follower_uuid: follower_uuid.clone(),
                            followed_uuid: followed_uuid.clone(),
                            username: &followed_user.username,
                            avatar_url: &followed_user.avatar_url,
                        };
                        diesel::insert_into(follow::table)
                            .values(records)
                            .execute(db)
                            .map(|_| DatabaseResponse::Ok)
                            .map_err(|err| {
                                eprintln!("Error following user: {}", err);
                                DatabaseResponse::Err(super::FollowError::InternalError)
                            })
                    })
                    .open_error()
            })
            .await
    }

    async fn un_follow_user(
        &self,
        follower_uuid: &str,
        followed_uuid: &str,
    ) -> DatabaseResponse<FollowError> {
        let follower_uuid = match Uuid::parse_str(follower_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return DatabaseResponse::Err(super::FollowError::UuidInvalid);
            }
        };
        let followed_uuid = match Uuid::parse_str(followed_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return DatabaseResponse::Err(super::FollowError::UserNotFound);
            }
        };
        self.0
            .run(move |db| {
                diesel::delete(
                    follow::table
                        .filter(follow::follower_uuid.eq(follower_uuid))
                        .filter(follow::followed_uuid.eq(followed_uuid)),
                )
                .execute(db)
                .map(|_| DatabaseResponse::Ok)
                .map_err(|err| {
                    eprintln!("Error unfollowing user: {}", err);
                    DatabaseResponse::Err(super::FollowError::InternalError)
                })
                .open_error()
            })
            .await
    }

    async fn is_following(
        &self,
        follower_uuid: &str,
        followed_uuid: &str,
    ) -> Result<bool, FollowError> {
        let follower_uuid = match Uuid::parse_str(follower_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FollowError::UuidInvalid);
            }
        };
        let followed_uuid = match Uuid::parse_str(followed_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(super::FollowError::UserNotFound);
            }
        };
        self.0
            .run(move |db| {
                follow::table
                    .filter(follow::follower_uuid.eq(follower_uuid))
                    .filter(follow::followed_uuid.eq(followed_uuid))
                    .first::<Follower>(db)
                    .map(|_| true)
                    .or_else(|_| Ok(false))
            })
            .await
    }

    async fn add_favourite(
        &self,
        uuid: &str,
        favourite_uuid: &str,
    ) -> DatabaseResponse<super::FavouriteError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return DatabaseResponse::Err(super::FavouriteError::UuidInvalid);
            }
        };
        let favourite_uuid = match Uuid::parse_str(favourite_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return DatabaseResponse::Err(super::FavouriteError::UserNotFound);
            }
        };
        self.0
            .run(move |db| {
                favourite::table
                    .filter(favourite::uuid.eq(uuid))
                    .filter(favourite::favourite_uuid.eq(favourite_uuid))
                    .first::<Favourite>(db)
                    .map(|_| DatabaseResponse::Err(super::FavouriteError::Conflict))
                    .or_else(|_| {
                        diesel::insert_into(favourite::table)
                            .values((
                                favourite::uuid.eq(uuid),
                                favourite::favourite_uuid.eq(favourite_uuid),
                            ))
                            .execute(db)
                            .map(|_| DatabaseResponse::Ok)
                            .map_err(|err| {
                                eprintln!("Error adding favourite: {}", err);
                                DatabaseResponse::Err(super::FavouriteError::InternalError)
                            })
                    })
                    .open_error()
            })
            .await
    }

    async fn remove_favourite(
        &self,
        uuid: &str,
        favourite_uuid: &str,
    ) -> DatabaseResponse<super::FavouriteError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return DatabaseResponse::Err(super::FavouriteError::UuidInvalid);
            }
        };
        let favourite_uuid = match Uuid::parse_str(favourite_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return DatabaseResponse::Err(super::FavouriteError::UserNotFound);
            }
        };

        self.0
            .run(move |db| {
                diesel::delete(
                    favourite::table
                        .filter(favourite::uuid.eq(uuid))
                        .filter(favourite::favourite_uuid.eq(favourite_uuid)),
                )
                .execute(db)
                .map(|_| DatabaseResponse::Ok)
                .map_err(|err| {
                    eprintln!("Error removing favourite: {}", err);
                    DatabaseResponse::Err(super::FavouriteError::InternalError)
                })
                .open_error()
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

#[derive(Debug)]
pub enum GetByUuidError {
    UuidInvalid,
    InternalError,
}

impl std::fmt::Display for GetByUuidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetByUuidError::UuidInvalid => write!(f, "UuidInvalid"),
            GetByUuidError::InternalError => write!(f, "InternalError"),
        }
    }
}
