use uuid::Uuid;

use crate::schema::matches;

#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct MatchesEntity {
    pub id: Uuid,
    pub creator_uuid: Uuid,
    pub user_id: Vec<Uuid>,
    pub title: String,
    pub url: String,
    pub description: String,
}

#[derive(Insertable, PartialEq, Debug, Clone)]
#[diesel(table_name = matches)]
pub struct MatchesEntityCreate {
    pub creator_uuid: Uuid,
    pub user_uuid: Vec<Uuid>,
    pub title: String,
    pub url: String,
    pub description: String,
}

#[derive(Debug)]
pub enum MatchesDbError {
    MatchesNotFound,
    MatchesNotCreated,
    UuidInvalid,
    InternalError,
}
