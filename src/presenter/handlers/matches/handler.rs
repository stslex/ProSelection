use crate::{
    data::repository::matches::MatchesRepository,
    presenter::handlers::objects::{
        request::{map_paging, PagingRequest},
        response::PagingResponse,
    },
    Conn,
};

use super::{
    objects::{UserCreateMatchRequest, UserMatchError, UserMatchResponse},
    MatchesHandler,
};
use crate::data::repository::matches::objects::MatchesDataCreate;

#[async_trait]
impl MatchesHandler for Conn {
    async fn create_match<'a>(
        &self,
        uuid: &'a str,
        params: UserCreateMatchRequest<'a>,
    ) -> Result<UserMatchResponse, UserMatchError> {
        if uuid != params.creator_uuid {
            return Err(UserMatchError::NoPermission);
        }
        let match_data = MatchesDataCreate {
            creator_uuid: params.creator_uuid,
            user_uuid: params.user_uuid,
            title: params.title,
            description: params.description,
            url: params.url,
        };
        self.create_matches(match_data)
            .await
            .map(|v| v.into())
            .map_err(|e| e.into())
    }
    async fn get_match<'a>(
        &self,
        user_uuid: &'a str,
        match_uuid: &'a str,
    ) -> Result<UserMatchResponse, UserMatchError> {
        self.get_current_match(user_uuid, match_uuid)
            .await
            .map_err(|e| e.into())
            .map(|v| v.into())
    }
    async fn get_matches<'a>(
        &self,
        uuid: &'a str,
        params: PagingRequest<'a>,
    ) -> Result<PagingResponse<UserMatchResponse>, UserMatchError> {
        let request = map_paging(uuid, params).await;
        MatchesRepository::get_matches(self, request)
            .await
            .map(|response| PagingResponse {
                page: response.page,
                total: response.total,
                has_more: response.has_more,
                page_size: response.page_size,
                result: response.result.into_iter().map(|v| v.into()).collect(),
            })
            .map_err(|e| e.into())
    }
}
