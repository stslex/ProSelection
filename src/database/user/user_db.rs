use super::{
    user_objects::{user::User, Favourite, Follower, NewFollow, UserCommonOutcome},
    FavouriteError, FollowError, UserDatabase,
};
use crate::{
    database::{Conn, DatabaseResponse},
    schema::{favourite, follow, users},
};
use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use rocket_sync_db_pools::diesel::Insertable;
use uuid::Uuid;

#[async_trait]
impl UserDatabase for Conn {
    async fn get_user_count(&self) -> UserCommonOutcome<String> {
        match users::table.get_results::<User>(&mut &self.0) {
            Ok(items) => UserCommonOutcome::Ok(items.len().to_string()),
            Err(_) => UserCommonOutcome::Error,
        }
    }

    async fn get_user(&self, uuid: &str) -> Result<User, GetByUuidError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(GetByUuidError::UuidInvalid);
            }
        };
        match users::table
            .filter(users::id.eq(uuid))
            .first::<User>(&self.0)
        {
            Ok(user) => Ok(user),
            Err(err) => {
                eprintln!("Error getting user: {}", err);
                Err(GetByUuidError::InternalError)
            }
        }
    }

    async fn get_user_by_username(&self, username: &str) -> Result<User, GetByUuidError> {
        match users::table
            .filter(users::username.eq(username))
            .first::<User>(&self.0)
        {
            Ok(user) => Ok(user),
            Err(err) => {
                eprintln!("Error getting user: {}", err);
                Err(GetByUuidError::InternalError)
            }
        }
    }

    async fn get_favourites_count(&self, uuid: &str) -> Result<i64, GetByUuidError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(GetByUuidError::UuidInvalid);
            }
        };
        match favourite::table
            .filter(favourite::uuid.eq(uuid))
            .count()
            .get_result::<i64>(&self.0)
        {
            Ok(count) => Ok(count),
            Err(err) => {
                eprintln!("Error getting user: {}", err);
                Err(GetByUuidError::InternalError)
            }
        }
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
        match follow::table
            .filter(follow::followed_uuid.eq(uuid))
            .count()
            .get_result::<i64>(&self.0)
        {
            Ok(count) => Ok(count),
            Err(err) => {
                eprintln!("Error getting user: {}", err);
                Err(GetByUuidError::InternalError)
            }
        }
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
        match follow::table
            .filter(follow::follower_uuid.eq(uuid))
            .count()
            .get_result::<i64>(&self.0)
        {
            Ok(count) => Ok(count),
            Err(err) => {
                eprintln!("Error getting user: {}", err);
                Err(GetByUuidError::InternalError)
            }
        }
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
        let followed_user = match users::table
            .filter(users::id.eq(followed_uuid))
            .first::<User>(&self.0)
        {
            Ok(user) => user,
            Err(err) => {
                eprintln!("Error getting user: {}", err);
                return DatabaseResponse::Err(super::FollowError::UserNotFound);
            }
        };
        let res = follow::table
            .filter(follow::follower_uuid.eq(follower_uuid))
            .filter(follow::followed_uuid.eq(followed_uuid))
            .first::<Follower>(&self.0)
            .map(|_| DatabaseResponse::Err(super::FollowError::Conflict))
            .or_else(|_| {
                let records = NewFollow {
                    follower_uuid: follower_uuid.clone(),
                    followed_uuid: followed_uuid.clone(),
                    username: &followed_user.username,
                    avatar_url: &followed_user.avatar_url,
                };
                diesel::insert_into(follow::table)
                    .values(records)
                    .execute(&mut &self.0)
                    .map(|_| DatabaseResponse::Ok)
                    .map_err(|err| {
                        eprintln!("Error following user: {}", err);
                        DatabaseResponse::Err(super::FollowError::InternalError)
                    })
            });
        match res {
            Ok(res) => res,
            Err(error) => {
                eprintln!("Error following user: {}", error);
                error
            }
        }
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
        let res = diesel::delete(
            follow::table
                .filter(follow::follower_uuid.eq(follower_uuid))
                .filter(follow::followed_uuid.eq(followed_uuid)),
        )
        .execute(&mut &self.0)
        .map(|_| DatabaseResponse::Ok)
        .map_err(|err| {
            eprintln!("Error unfollowing user: {}", err);
            DatabaseResponse::Err(super::FollowError::InternalError)
        });
        match res {
            Ok(res) => res,
            Err(error) => {
                eprintln!("Error unfollowing user: {}", error);
                error
            }
        }
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
        follow::table
            .filter(follow::follower_uuid.eq(follower_uuid))
            .filter(follow::followed_uuid.eq(followed_uuid))
            .first::<Follower>(&self.0)
            .map(|_| true)
            .or_else(|_| Ok(false))
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
        let res = favourite::table
            .filter(favourite::uuid.eq(uuid))
            .filter(favourite::favourite_uuid.eq(favourite_uuid))
            .first::<Favourite>(&self.0)
            .map(|_| DatabaseResponse::Err(super::FavouriteError::Conflict))
            .or_else(|_| {
                diesel::insert_into(favourite::table)
                    .values((
                        favourite::uuid.eq(uuid),
                        favourite::favourite_uuid.eq(favourite_uuid),
                    ))
                    .execute(&mut &self.0)
                    .map(|_| DatabaseResponse::Ok)
                    .map_err(|err| {
                        eprintln!("Error adding favourite: {}", err);
                        DatabaseResponse::Err(super::FavouriteError::InternalError)
                    })
            });
        match res {
            Ok(res) => res,
            Err(error) => {
                eprintln!("Error adding favourite: {}", error);
                error
            }
        }
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
        let res = diesel::delete(
            favourite::table
                .filter(favourite::uuid.eq(uuid))
                .filter(favourite::favourite_uuid.eq(favourite_uuid)),
        )
        .execute(&mut &self.0)
        .map(|_| DatabaseResponse::Ok)
        .map_err(|err| {
            eprintln!("Error removing favourite: {}", err);
            DatabaseResponse::Err(super::FavouriteError::InternalError)
        });
        match res {
            Ok(res) => res,
            Err(error) => {
                eprintln!("Error removing favourite: {}", error);
                error
            }
        }
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
        favourite::table
            .filter(favourite::user_uuid.eq(uuid))
            .filter(favourite::favourite_uuid.eq(favourite_uuid))
            .first::<Favourite>(&self.0)
            .map(|_| true)
            .or_else(|_| Ok(false))
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
