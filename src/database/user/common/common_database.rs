use diesel::associations::HasTable;
use diesel::RunQueryDsl;

use crate::database::user::common::{UserCommonDatabase, UserCommonOutcome};
use crate::database::user::user_objects::user::User;
use crate::database::Conn;
use crate::schema::users::dsl::users;

impl UserCommonDatabase for Conn {
    fn get_user_count(&self) -> UserCommonOutcome<String> {
        match users::table().get_results::<User>(&self.0) {
            Ok(items) => UserCommonOutcome::Ok(items.len().to_string()),
            Err(_) => UserCommonOutcome::Other,
        }
    }
}
