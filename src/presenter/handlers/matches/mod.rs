use self::objects::{UserCreateMatchRequest, UserMatchError, UserMatchResponse};

mod handler;
pub mod objects;

#[async_trait]
pub trait MatchesHandler {
    async fn create_match<'a>(
        &self,
        uuid: &'a str,
        params: UserCreateMatchRequest<'a>,
    ) -> Result<UserMatchResponse, UserMatchError>;
}
