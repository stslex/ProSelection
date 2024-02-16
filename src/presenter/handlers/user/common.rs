use crate::{
    data::{database::user::objects::UserDataError, repository::user::UserRepository},
    Conn,
};

pub async fn count(db: Conn) -> Result<String, CommonError> {
    db.get_user_count().await.map_err(|err| match err {
        UserDataError::InternalError => CommonError::Other,
        UserDataError::UuidInvalid => CommonError::UuidInvalid,
    })
}

pub enum CommonError {
    Other,
    UuidInvalid,
}
