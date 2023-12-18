use crate::database::{
    self,
    user::{user_objects::UserCommonOutcome, UserDatabase},
};

pub fn count(db: database::Conn) -> Result<String, CommonError> {
    match db.get_user_count() {
        UserCommonOutcome::Ok(s) => Ok(s),
        UserCommonOutcome::Error => Err(CommonError::Other),
    }
}

pub enum CommonError {
    Other,
}
