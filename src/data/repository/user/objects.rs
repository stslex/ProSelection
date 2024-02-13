use rocket::futures;
use uuid::Uuid;

use crate::{data::database::user::objects::UserEntity, utils::Mapper};

pub struct UserDataResponse {
    pub id: Uuid,
    pub login: String,
    pub username: String,
    pub secret: String,
    pub avatar_url: String,
    pub bio: String,
}

#[async_trait]
impl Mapper<UserDataResponse> for UserEntity {
    async fn map(&self) -> UserDataResponse {
        UserDataResponse {
            id: self.id,
            login: self.login.clone(),
            username: self.username.clone(),
            secret: self.secret.clone(),
            avatar_url: self.avatar_url.clone(),
            bio: self.bio.clone(),
        }
    }
}

#[async_trait]
impl Mapper<Vec<UserDataResponse>> for Vec<UserEntity> {
    async fn map(&self) -> Vec<UserDataResponse> {
        futures::future::join_all(self.into_iter().map(|user| user.map())).await
    }
}
