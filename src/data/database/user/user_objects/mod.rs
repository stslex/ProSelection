pub mod user;

use crate::schema::follow;
use rocket_sync_db_pools::diesel::Insertable;
use uuid::Uuid;

/// Represents the outcome of common user operations.
pub enum UserCommonOutcome<T> {
    Ok(T),
    Error,
}

/// Represents a new following relationship to be inserted into the database.
#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = follow)]
pub struct NewFollow<'a> {
    pub follower_uuid: Uuid,
    pub followed_uuid: Uuid,
    pub username: &'a str,
    pub avatar_url: &'a str,
}

/// Represents a follower object retrieved from the database.
#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct Follower {
    pub uuid: Uuid,
    pub follower_uuid: Uuid,
    pub followed_uuid: Uuid,
    pub username: String,
    pub avatar_url: String,
}
