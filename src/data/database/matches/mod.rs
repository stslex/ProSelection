use crate::data::repository::objects::{PagingDomainRequest, PagingDomainResponse};

use self::objects::{MatchesDbError, MatchesEntity, MatchesEntityCreate};

mod database;
pub mod objects;
mod tests;

#[async_trait]
pub trait MatchesDatabase {
    async fn get_match(&self, id: String) -> Result<MatchesEntity, MatchesDbError>;
    async fn add_match(
        &self,
        match_entity: MatchesEntityCreate,
    ) -> Result<MatchesEntity, MatchesDbError>;
    async fn get_matches<'a>(
        &self,
        request: PagingDomainRequest<'a>,
    ) -> Result<PagingDomainResponse<MatchesEntity>, MatchesDbError>;
    async fn get_match_count<'a>(&self, user_uuid: &'a str) -> Result<i64, MatchesDbError>;
}
