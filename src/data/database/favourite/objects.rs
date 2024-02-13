use uuid::Uuid;

use crate::schema::favourite;

/// Represents a new favorite item to be inserted into the database.
#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = favourite)]
pub struct FavouriteEntity {
    pub user_uuid: Uuid,
    pub favourite_uuid: Uuid,
    pub title: String,
}

// Represents a favourite object retrieved from the database.
#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct FavouriteEntityResponse {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub favourite_uuid: Uuid,
    pub title: String,
}

// Represents a favourite object retrieved from the database.
#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct FavouriteDataResponse {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub favourite_uuid: Uuid,
    pub title: String,
}

pub struct FavouriteDbSearchRequest<'a> {
    pub request_uuid: &'a str,
    pub uuid: &'a str,
    pub query: &'a str,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone)]
pub enum FavouriteDbError {
    UuidInvalid,
    UserNotFound,
    Conflict,
    InternalError,
}
