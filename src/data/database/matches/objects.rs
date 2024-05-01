use uuid::Uuid;

use crate::schema::matches;

#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct MatchesEntity {
    pub uuid: Uuid,
    pub creator_uuid: Uuid,
    pub participants_uuid: Vec<Uuid>,
    pub title: String,
    pub description: String,
    pub status: String,
    pub cover_url: String,
    pub expires_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Insertable, PartialEq, Debug, Clone)]
#[diesel(table_name = matches)]
pub struct MatchesEntityCreate {
    pub creator_uuid: Uuid,
    pub participants_uuid: Vec<Uuid>,
    pub title: String,
    pub description: String,
    pub cover_url: String,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub expires_at: i64,
}

#[derive(Debug)]
pub enum MatchesDbError {
    MatchesNotFound,
    MatchesNotCreated,
    UuidInvalid,
    InternalError,
}
