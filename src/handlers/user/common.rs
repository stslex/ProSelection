use crate::database;
use crate::database::user::common::{UserCommonDatabase, UserCommonOutcome};

pub fn count(db: database::Conn) -> Result<String, CommonError> {
    match db.get_user_count() {
        UserCommonOutcome::Ok(s) => Ok(s),
        UserCommonOutcome::Other => Err(CommonError::Other),
    }
}

pub enum CommonError {
    Other,
}