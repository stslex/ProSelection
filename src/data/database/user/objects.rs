use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct UserEntity {
    pub id: Uuid,
    pub login: String,
    pub username: String,
    pub secret: String,
    pub avatar_url: String,
    pub bio: String,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct UserEntityCreate {
    pub login: String,
    pub username: String,
    pub secret: String,
    pub avatar_url: String,
    pub bio: String,
}

#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct Followers {
    pub uuid: Uuid,
    pub follower_uuid: Uuid,
    pub username: String,
    pub avatar_url: String,
}

#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct Following {
    pub uuid: Uuid,
    pub following_uuid: Uuid,
    pub username: String,
    pub avatar_url: String,
}
