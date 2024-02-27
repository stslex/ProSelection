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
}
