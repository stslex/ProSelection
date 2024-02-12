use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    data::database::{user::objects::UserEntity, Conn},
    schema::{follow, users},
};

use super::{
    objects::{FollowDataError, FollowEntityCreate, FollowPagingDataRequest, FollowerEntity},
    FollowDatabase,
};

#[async_trait]
impl FollowDatabase for Conn {
    async fn get_followers_count<'a>(&self, uuid: &'a str) -> Result<i64, FollowDataError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(FollowDataError::UuidInvalid);
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
                        Err(FollowDataError::InternalError)
                    }
                }
            })
            .await
    }

    async fn get_following_count<'a>(&self, uuid: &'a str) -> Result<i64, FollowDataError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(FollowDataError::UuidInvalid);
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
                        Err(FollowDataError::InternalError)
                    }
                }
            })
            .await
    }

    async fn follow_user<'a>(
        &self,
        follower_uuid: &'a str,
        followed_uuid: &'a str,
    ) -> Result<(), FollowDataError> {
        let follower_uuid = match Uuid::parse_str(follower_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(FollowDataError::UuidInvalid);
            }
        };
        let followed_uuid = match Uuid::parse_str(followed_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(FollowDataError::UserNotFound);
            }
        };
        self.0
            .run(move |db| {
                let followed_user = match users::table
                    .filter(users::id.eq(followed_uuid))
                    .first::<UserEntity>(db)
                {
                    Ok(user) => user,
                    Err(err) => {
                        eprintln!("Error getting user: {}", err);
                        return Result::Err(FollowDataError::UserNotFound);
                    }
                };
                match follow::table
                    .filter(follow::follower_uuid.eq(follower_uuid))
                    .filter(follow::followed_uuid.eq(followed_uuid))
                    .first::<FollowerEntity>(db)
                    .map(|_: FollowerEntity| Result::Err(FollowDataError::Conflict))
                    .or_else(|_| {
                        let records = FollowEntityCreate {
                            follower_uuid: follower_uuid.clone(),
                            followed_uuid: followed_uuid.clone(),
                            username: &followed_user.username,
                            avatar_url: &followed_user.avatar_url,
                        };
                        diesel::insert_into(follow::table)
                            .values(records)
                            .execute(db)
                            .map(|_| Result::Ok(()))
                            .map_err(|err| {
                                eprintln!("Error following user: {}", err);
                                Result::Err(FollowDataError::InternalError)
                            })
                    }) {
                    Ok(result) => result,
                    Err(err) => err,
                }
            })
            .await
    }

    async fn un_follow_user<'a>(
        &self,
        follower_uuid: &'a str,
        followed_uuid: &'a str,
    ) -> Result<(), FollowDataError> {
        let follower_uuid = match Uuid::parse_str(follower_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(FollowDataError::UuidInvalid);
            }
        };
        let followed_uuid = match Uuid::parse_str(followed_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(FollowDataError::UserNotFound);
            }
        };
        self.0
            .run(move |db| {
                match diesel::delete(
                    follow::table
                        .filter(follow::follower_uuid.eq(follower_uuid))
                        .filter(follow::followed_uuid.eq(followed_uuid)),
                )
                .execute(db)
                .map(|_| Result::Ok(()))
                .map_err(|err| {
                    eprintln!("Error unfollowing user: {}", err);
                    Result::Err(FollowDataError::InternalError)
                }) {
                    Ok(result) => result,
                    Err(err) => err,
                }
            })
            .await
    }

    async fn is_following<'a>(
        &self,
        follower_uuid: &'a str,
        followed_uuid: &'a str,
    ) -> Result<bool, FollowDataError> {
        let follower_uuid = match Uuid::parse_str(follower_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(FollowDataError::UuidInvalid);
            }
        };
        let followed_uuid = match Uuid::parse_str(followed_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Result::Err(FollowDataError::UserNotFound);
            }
        };
        self.0
            .run(move |db| {
                follow::table
                    .filter(follow::follower_uuid.eq(follower_uuid))
                    .filter(follow::followed_uuid.eq(followed_uuid))
                    .first::<FollowerEntity>(db)
                    .map(|_| true)
                    .or_else(|_| Ok(false))
            })
            .await
    }

    async fn get_user_following<'a>(
        &self,
        request: &FollowPagingDataRequest<'a>,
    ) -> Result<Vec<FollowerEntity>, FollowDataError> {
        let uuid = match Uuid::parse_str(request.uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(FollowDataError::UuidInvalid);
            }
        };
        let limit = request.page_size;
        let offset = request.page * request.page_size;
        self.0
            .run(move |db| {
                let users: Vec<FollowerEntity> = follow::table
                    .filter(follow::follower_uuid.eq(uuid))
                    .limit(limit)
                    .offset(offset)
                    .get_results::<FollowerEntity>(db)
                    .map_err(|err| {
                        eprintln!("Error getting users: {}", err);
                        FollowDataError::InternalError
                    })?
                    .into_iter()
                    .collect();
                Ok(users)
            })
            .await
    }

    async fn get_user_followers<'a>(
        &self,
        request: &FollowPagingDataRequest<'a>,
    ) -> Result<Vec<FollowerEntity>, FollowDataError> {
        let uuid = match Uuid::parse_str(request.uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(FollowDataError::UuidInvalid);
            }
        };
        let request_uuid = match Uuid::parse_str(request.request_uuid) {
            Ok(uuid) => uuid,
            Err(err) => {
                eprintln!("Error parsing uuid: {}", err);
                return Err(FollowDataError::UuidInvalid);
            }
        };
        let limit = request.page_size;
        let offset = request.page * request.page_size;
        self.0
            .run(move |db| {
                let users: Vec<FollowerEntity> = follow::table
                    .filter(follow::followed_uuid.eq(uuid))
                    .filter(follow::followed_uuid.ne(request_uuid))
                    .limit(limit)
                    .offset(offset)
                    .get_results::<FollowerEntity>(db)
                    .map_err(|err| {
                        eprintln!("Error getting users: {}", err);
                        FollowDataError::InternalError
                    })?
                    .into_iter()
                    .collect();
                Ok(users)
            })
            .await
    }
}
