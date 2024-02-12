use uuid::Uuid;

use crate::schema::follow;

#[derive(Debug)]
pub enum UserSearchError {
    UuidInvalid,
    InternalError,
}

/// Represents a new following relationship to be inserted into the database.
#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = follow)]
pub struct FollowEntityCreate<'a> {
    pub follower_uuid: Uuid,
    pub followed_uuid: Uuid,
    pub username: &'a str,
    pub avatar_url: &'a str,
}

/// Represents a follower object retrieved from the database.
#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct FollowerEntity {
    pub uuid: Uuid,
    pub follower_uuid: Uuid,
    pub followed_uuid: Uuid,
    pub username: String,
    pub avatar_url: String,
}

pub struct FollowPagingDataRequest<'a> {
    pub request_uuid: &'a str,
    pub uuid: &'a str,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone)]
pub enum FollowDataError {
    UuidInvalid,
    UserNotFound,
    Conflict,
    InternalError,
}

impl std::fmt::Display for FollowDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FollowDataError::UuidInvalid => write!(f, "UuidInvalid"),
            FollowDataError::UserNotFound => write!(f, "UserNotFound"),
            FollowDataError::Conflict => write!(f, "Conflict"),
            FollowDataError::InternalError => write!(f, "InternalError"),
        }
    }
}
