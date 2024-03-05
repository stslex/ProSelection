use self::objects::{MatchesData, MatchesDataCreate, MatchesDataError};

use super::objects::{PagingDomainRequest, PagingDomainResponse};

pub mod objects;
mod repository;

#[async_trait]
pub trait MatchesRepository {
    async fn create_matches<'a>(
        &self,
        request: MatchesDataCreate<'a>,
    ) -> Result<MatchesData, MatchesDataError>;
    async fn get_current_match<'a>(
        &self,
        request_uuid: &'a str,
        match_uuid: &'a str,
    ) -> Result<MatchesData, MatchesDataError>;
    async fn get_matches<'a>(
        &self,
        request: PagingDomainRequest<'a>,
    ) -> Result<PagingDomainResponse<MatchesData>, MatchesDataError>;
    async fn get_match_count<'a>(
        &self,
        user_uuid: &'a str,
        request_uuid: &'a str,
    ) -> Result<i64, MatchesDataError>;
}
