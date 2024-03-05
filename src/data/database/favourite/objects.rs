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

#[derive(Debug, Clone)]
pub enum FavouriteDbError {
    UuidInvalid,
    UserNotFound,
    Conflict,
    InternalError,
}
