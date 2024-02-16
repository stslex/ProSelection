use uuid::Uuid;

use crate::schema::follow;

/// Represents a new following relationship to be inserted into the database.
#[derive(Insertable, PartialEq, Debug, Clone)]
#[diesel(table_name = follow)]
pub struct FollowEntityCreate {
    pub follower_uuid: Uuid,
    pub followed_uuid: Uuid,
    pub username: String,
    pub avatar_url: String,
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
