use crate::{
    data::repository::matches::MatchesRepository,
    presenter::handlers::objects::{
        request::{map_paging_uuid, PagingUuidRequest},
        response::PagingResponse,
    },
    Conn,
};

use super::{
    objects::{UserCreateMatchRequest, UserMatchDetailResponse, UserMatchError},
    MatchesHandler,
};
use crate::data::repository::matches::objects::MatchesDataCreate;

#[async_trait]
impl MatchesHandler for Conn {
    async fn create_match<'a>(
        &self,
        uuid: &'a str,
        params: UserCreateMatchRequest<'a>,
    ) -> Result<UserMatchDetailResponse, UserMatchError> {
        let current_time_ms = chrono::Utc::now().timestamp_millis();
        let match_data = MatchesDataCreate {
            creator_uuid: uuid,
            participants_uuid: params.participants_uuid,
            title: params.title,
            description: params.description,
            cover_url: params.cover_url,
            created_at: current_time_ms,
            expires_at: current_time_ms,
            updated_at: current_time_ms,
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
    ) -> Result<UserMatchDetailResponse, UserMatchError> {
        self.get_current_match(user_uuid, match_uuid)
            .await
            .map_err(|e| e.into())
            .map(|v| v.into())
    }
    async fn get_matches<'a>(
        &self,
        uuid: &'a str,
        params: PagingUuidRequest<'a>,
    ) -> Result<PagingResponse<UserMatchDetailResponse>, UserMatchError> {
        let request = map_paging_uuid(uuid, params).await;
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
