use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    data::{
        database::utils::paging::correct_page_number,
        repository::{
            follow::objects::FollowDataError,
            objects::{PagingDomainRequest, PagingDomainResponse},
        },
    },
    schema::follow,
    Conn,
};

use super::{
    objects::{FollowEntityCreate, FollowerEntity},
    FollowDatabase,
};

#[async_trait]
impl FollowDatabase for Conn {
    async fn get_followers_count<'a>(&self, uuid: &'a str) -> Result<i64, FollowDataError> {
        let uuid = Uuid::parse_str(uuid).map_err(|_| (FollowDataError::UuidInvalid))?;
        let query = follow::table.filter(follow::followed_uuid.eq(uuid)).count();
        self.0
            .run(move |db| {
                query.get_result::<i64>(db).map_err(|err| {
                    eprintln!("Error getting user: {}", err);
                    FollowDataError::InternalError
                })
            })
            .await
    }

    async fn get_following_count<'a>(&self, uuid: &'a str) -> Result<i64, FollowDataError> {
        let uuid = Uuid::parse_str(uuid).map_err(|_| (FollowDataError::UuidInvalid))?;
        let query_builder = follow::table.filter(follow::follower_uuid.eq(uuid)).count();
        self.0
            .run(move |db| {
                query_builder.get_result::<i64>(db).map_err(|err| {
                    eprintln!("Error getting user: {}", err);
                    FollowDataError::InternalError
                })
            })
            .await
    }

    async fn follow_user<'a>(
        &self,
        record: &'a FollowEntityCreate,
    ) -> Result<FollowerEntity, FollowDataError> {
        let record = record.to_owned();
        if self
            .is_following_uuid(&record.follower_uuid, &record.followed_uuid)
            .await?
        {
            return Result::Err(FollowDataError::Conflict);
        };
        self.0
            .run(move |db| {
                diesel::insert_into(follow::table)
                    .values(record)
                    .get_result(db)
            })
            .await
            .map_err(|err| {
                eprintln!("Error following user: {}", err);
                FollowDataError::InternalError
            })
    }

    async fn un_follow_user<'a>(
        &self,
        follower_uuid: &'a str,
        followed_uuid: &'a str,
    ) -> Result<(), FollowDataError> {
        let follower_uuid =
            Uuid::parse_str(follower_uuid).map_err(|_| (FollowDataError::UuidInvalid))?;
        let followed_uuid =
            Uuid::parse_str(followed_uuid).map_err(|_| (FollowDataError::UuidInvalid))?;

        let query_builder = follow::table
            .filter(follow::follower_uuid.eq(follower_uuid))
            .filter(follow::followed_uuid.eq(followed_uuid));
        self.0
            .run(move |db| {
                diesel::delete(query_builder)
                    .execute(db)
                    .map(|_| ())
                    .map_err(|err| {
                        eprintln!("Error unfollowing user: {}", err);
                        FollowDataError::InternalError
                    })
            })
            .await
    }

    async fn is_following<'a>(
        &self,
        follower_uuid: &'a str,
        followed_uuid: &'a str,
    ) -> Result<bool, FollowDataError> {
        let follower_uuid =
            Uuid::parse_str(follower_uuid).map_err(|_| (FollowDataError::UuidInvalid))?;
        let followed_uuid =
            Uuid::parse_str(followed_uuid).map_err(|_| (FollowDataError::UuidInvalid))?;
        self.is_following_uuid(&follower_uuid, &followed_uuid).await
    }

    async fn is_following_uuid<'a>(
        &self,
        follower_uuid: &'a Uuid,
        followed_uuid: &'a Uuid,
    ) -> Result<bool, FollowDataError> {
        let follower_uuid = *follower_uuid;
        let followed_uuid = *followed_uuid;
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
        request: &'a PagingDomainRequest<'a>,
    ) -> Result<PagingDomainResponse<FollowerEntity>, FollowDataError> {
        let uuid =
            Uuid::parse_str(request.user_uuid).map_err(|_| (FollowDataError::UuidInvalid))?;

        let query = request.query.to_owned().to_lowercase();
        let page_number = correct_page_number(request.page);
        let limit = request.page_size;
        let offset = page_number * request.page_size;
        let page_size = request.page_size;

        self.0
            .run(move |db| {
                let query_request = follow::table
                    .filter(follow::follower_uuid.eq(uuid))
                    .filter(follow::follower_username.ilike(format!("%{}%", query)));

                let results: Vec<FollowerEntity> = query_request
                    .to_owned()
                    .limit(limit)
                    .offset(offset)
                    .get_results::<FollowerEntity>(db)
                    .map_err(|err| {
                        eprintln!("Error getting users: {}", err);
                        FollowDataError::InternalError
                    })?
                    .into_iter()
                    .collect();
                let total_request = query_request.to_owned();
                let total_result = total_request.count().get_result(db).map_err(|err| {
                    println!("Error getting total count: {}", err);
                    FollowDataError::InternalError
                })?;

                let result_count = i64::try_from(results.len()).map_err(|err| {
                    eprintln!("Error converting result count: {}", err);
                    FollowDataError::InternalError
                })?;

                Ok(PagingDomainResponse {
                    total: total_result,
                    result: results,
                    page: page_number + 1,
                    page_size: page_size,
                    has_more: offset + result_count < total_result,
                })
            })
            .await
    }

    async fn get_user_followers<'a>(
        &self,
        request: &'a PagingDomainRequest<'a>,
    ) -> Result<PagingDomainResponse<FollowerEntity>, FollowDataError> {
        let uuid =
            Uuid::parse_str(request.user_uuid).map_err(|_| (FollowDataError::UuidInvalid))?;
        let request_uuid =
            Uuid::parse_str(request.request_uuid).map_err(|_| (FollowDataError::UuidInvalid))?;

        let query = request.query.to_owned().to_lowercase();
        let page_number = correct_page_number(request.page);
        let limit = request.page_size;
        let offset = page_number * request.page_size;
        let page_size = request.page_size;

        self.0
            .run(move |db| {
                let query_request = follow::table
                    .filter(follow::followed_uuid.eq(uuid))
                    .filter(follow::follower_uuid.ne(request_uuid))
                    .filter(follow::follower_username.ilike(format!("%{}%", query)));

                let results: Vec<FollowerEntity> = query_request
                    .to_owned()
                    .limit(limit)
                    .offset(offset)
                    .get_results::<FollowerEntity>(db)
                    .map_err(|err| {
                        eprintln!("Error getting users: {}", err);
                        FollowDataError::InternalError
                    })?
                    .into_iter()
                    .collect();

                let total_request = query_request.to_owned();
                let total_result = total_request.count().get_result(db).map_err(|err| {
                    println!("Error getting total count: {}", err);
                    FollowDataError::InternalError
                })?;

                let result_count = i64::try_from(results.len()).map_err(|err| {
                    eprintln!("Error converting result count: {}", err);
                    FollowDataError::InternalError
                })?;

                Ok(PagingDomainResponse {
                    total: total_result,
                    result: results,
                    page: page_number + 1,
                    page_size: page_size,
                    has_more: offset + result_count < total_result,
                })
            })
            .await
    }
}
