use crate::{data::repository::matches::MatchesRepository, Conn};

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
}
