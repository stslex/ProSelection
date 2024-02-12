use crate::data::database::{
    self,
    user::{objects::UserDataError, UserDatabase},
};

pub async fn count(db: database::Conn) -> Result<String, CommonError> {
    db.get_user_count().await.map_err(|err| match err {
        UserDataError::InternalError => CommonError::Other,
        UserDataError::UuidInvalid => CommonError::UuidInvalid,
    })
}

pub enum CommonError {
    Other,
    UuidInvalid,
}
