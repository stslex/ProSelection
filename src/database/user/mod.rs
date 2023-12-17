use self::{
    user_db::GetUserError,
    user_objects::{user::User, UserCommonOutcome},
};

pub mod user_db;
pub mod user_objects;

pub trait UserDatabase {
    fn get_user_count(&self) -> UserCommonOutcome<String>;
    fn get_user(&self, uuid: &str) -> Result<User, GetUserError>;
    fn get_user_by_username(&self, username: &str) -> Result<User, GetUserError>;
}
