use std::ops::Not;

use crate::{
    data::{
        database::{
            follow::FollowDatabase,
            matches::{objects::MatchesEntityCreate, MatchesDatabase},
            user::UserDatabase,
        },
        repository::objects::{PagingDomainRequest, PagingDomainResponse},
    },
    utils::Mapper,
    Conn,
};

use super::{
    objects::{MatchStatus, MatchesData, MatchesDataCreate, MatchesDataError},
    MatchesRepository,
};

#[async_trait]
impl MatchesRepository for Conn {
    async fn create_matches<'a>(
        &self,
        request: MatchesDataCreate<'a>,
    ) -> Result<MatchesData, MatchesDataError> {
        let match_entity = MatchesEntityCreate {
            creator_uuid: request.creator_uuid.map().await?,
            participants_uuid: request.participants_uuid.map().await?,
            title: request.title.to_owned(),
            description: request.description.to_owned(),
            cover_url: request.cover_url.to_owned(),
            status: MatchStatus::Pending.into(),
            created_at: request.created_at,
            updated_at: request.updated_at,
            expires_at: request.expires_at,
        };
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
        if match_entity.participants_uuid.contains(&user.id).not() {
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
