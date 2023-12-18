use self::{
    user_db::GetByUuidError,
    user_objects::{user::User, UserCommonOutcome},
};

pub mod user_db;
pub mod user_objects;

pub trait UserDatabase {
    fn get_user_count(&self) -> UserCommonOutcome<String>;
    fn get_user(&self, uuid: &str) -> Result<User, GetByUuidError>;
    fn get_user_by_username(&self, username: &str) -> Result<User, GetByUuidError>;
    fn get_favourites_count(&self, uuid: &str) -> Result<i64, GetByUuidError>;
    fn get_followers_count(&self, uuid: &str) -> Result<i64, GetByUuidError>;
    fn get_following_count(&self, uuid: &str) -> Result<i64, GetByUuidError>;
}
