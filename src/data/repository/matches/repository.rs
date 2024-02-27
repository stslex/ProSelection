use crate::{
    data::{
        database::matches::{
            objects::{MatchesDbError, MatchesEntity},
            MatchesDatabase,
        },
        repository::user::UserRepository,
    },
    utils::Mapper,
    Conn,
};
use rocket::futures;

use super::{
    objects::{MatchesData, MatchesDataCreate, MatchesDataError, MatchesDataRequest},
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
    async fn get_matches<'a>(
        &self,
        request: MatchesDataRequest<'a>,
    ) -> Result<Vec<MatchesData>, MatchesDataError> {
        let user = self
            .get_user(request.user_uuid)
            .await
            .map_err(|_| MatchesDataError::MatchesNotFound)?;

        futures::future::join_all(
            user.matches
                .into_iter()
                .map(|match_uuid| self.get_match(match_uuid.to_string())),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<MatchesEntity>, MatchesDbError>>()
        .map_err(|err| err.into())?
        .into_iter()
        .map(|value| Result::Ok(value.into()))
        .collect::<Result<Vec<MatchesData>, MatchesDataError>>()
    }
}
