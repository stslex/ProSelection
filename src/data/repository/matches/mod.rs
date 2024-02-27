use self::objects::{MatchesData, MatchesDataCreate, MatchesDataError, MatchesDataRequest};

mod objects;
mod repository;

#[async_trait]
pub trait MatchesRepository {
    async fn create_matches<'a>(
        &self,
        request: MatchesDataCreate<'a>,
    ) -> Result<MatchesData, MatchesDataError>;
    async fn get_matches<'a>(
        &self,
        request: MatchesDataRequest<'a>,
    ) -> Result<Vec<MatchesData>, MatchesDataError>;
}
