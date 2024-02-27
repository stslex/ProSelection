use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{schema::matches, Conn};

use super::{
    objects::{MatchesDbError, MatchesEntity, MatchesEntityCreate},
    MatchesDatabase,
};

#[async_trait]
impl MatchesDatabase for Conn {
    async fn get_match(&self, id: String) -> Result<MatchesEntity, MatchesDbError> {
        let matches_id = Uuid::parse_str(&id).map_err(|_| MatchesDbError::UuidInvalid)?;
        self.0
            .run(move |db| {
                matches::table
                    .filter(matches::id.eq(matches_id))
                    .first::<MatchesEntity>(db)
                    .map_err(|err| {
                        println!("{:?}", err);
                        match err {
                            diesel::result::Error::NotFound => MatchesDbError::MatchesNotFound,
                            _ => MatchesDbError::InternalError,
                        }
                    })
            })
            .await
    }
    async fn add_match(
        &self,
        match_entity: MatchesEntityCreate,
    ) -> Result<MatchesEntity, MatchesDbError> {
        self.0
            .run(move |db| {
                diesel::insert_into(matches::table)
                    .values(&match_entity)
                    .get_result::<MatchesEntity>(db)
                    .map_err(|err| {
                        println!("{:?}", err);
                        match err {
                            diesel::result::Error::NotFound => MatchesDbError::MatchesNotCreated,
                            _ => MatchesDbError::InternalError,
                        }
                    })
            })
            .await
    }
}
