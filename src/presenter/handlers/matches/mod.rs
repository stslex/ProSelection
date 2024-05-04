use self::objects::{UserCreateMatchRequest, UserMatchDetailResponse, UserMatchError};
use super::objects::{request::PagingUuidRequest, response::PagingResponse};
mod handler;
pub mod objects;

#[async_trait]
pub trait MatchesHandler {
    async fn create_match<'a>(
        &self,
        uuid: &'a str,
        params: UserCreateMatchRequest<'a>,
    ) -> Result<UserMatchDetailResponse, UserMatchError>;

    async fn get_match<'a>(
        &self,
        user_uuid: &'a str,
        match_uuid: &'a str,
    ) -> Result<UserMatchDetailResponse, UserMatchError>;

    async fn get_matches<'a>(
        &self,
        uuid: &'a str,
        params: PagingUuidRequest<'a>,
    ) -> Result<PagingResponse<UserMatchDetailResponse>, UserMatchError>;
}
