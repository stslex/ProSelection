use std::ops::Not;

use crate::{
    data::{
        database::{follow::FollowDatabase, matches::MatchesDatabase, user::UserDatabase},
        repository::objects::{PagingDomainRequest, PagingDomainResponse},
    },
    utils::Mapper,
    Conn,
};

use super::{
    objects::{MatchesData, MatchesDataCreate, MatchesDataError},
    MatchesRepository,
};

#[async_trait]
impl MatchesRepository for Conn {
    async fn create_matches<'a>(
        &self,
        request: MatchesDataCreate<'a>,
    ) -> Result<MatchesData, MatchesDataError> {
        let match_entity = request.map().await?;
        self.add_match(match_entity)
            .await
            .map_err(|value| value.into())
            .map(|value| value.into())
    }
    async fn get_current_match<'a>(
        &self,
        request_uuid: &'a str,
        match_uuid: &'a str,
    ) -> Result<MatchesData, MatchesDataError> {
        let user = self
            .get_user(request_uuid)
            .await
            .map_err(|_| MatchesDataError::MatchesNotFound)?;
        let match_entity = self
            .get_match(match_uuid.to_string())
            .await
            .map_err(|_| MatchesDataError::MatchesNotFound)?;
        if match_entity.user_id.contains(&user.id).not() {
            println!("User not found in match");
            Result::Err(MatchesDataError::NoPermission)
        } else {
            Result::Ok(match_entity.into())
        }
    }

    async fn get_matches<'a>(
        &self,
        request: PagingDomainRequest<'a>,
    ) -> Result<PagingDomainResponse<MatchesData>, MatchesDataError> {
        let is_permitted = if request.user_uuid == request.request_uuid {
            true
        } else {
            self.is_following(request.user_uuid, request.request_uuid)
                .await
                .map(|value| value)
                .map_err(|_| MatchesDataError::MatchesNotFound)?
        };

        if is_permitted.not() {
            return Result::Err(MatchesDataError::NoPermission);
        }

        MatchesDatabase::get_matches(self, request)
            .await
            .map(|response| PagingDomainResponse {
                page: response.page,
                page_size: response.page_size,
                total: response.total,
                has_more: response.has_more,
                result: response.result.into_iter().map(|v| v.into()).collect(),
            })
            .map_err(|value| value.into())
    }

    async fn get_match_count<'a>(
        &self,
        user_uuid: &'a str,
        request_uuid: &'a str,
    ) -> Result<i64, MatchesDataError> {
        let is_permitted = if request_uuid == user_uuid {
            true
        } else {
            self.is_following(user_uuid, request_uuid)
                .await
                .map(|value| value)
                .map_err(|_| MatchesDataError::MatchesNotFound)?
        };

        if is_permitted.not() {
            return Result::Err(MatchesDataError::NoPermission);
        }

        MatchesDatabase::get_match_count(self, user_uuid)
            .await
            .map(|value| value)
            .map_err(|value| value.into())
    }
}
