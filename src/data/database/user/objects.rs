use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct UserEntity {
    #[diesel(column_name = "id")]
    pub id: Uuid,
    #[diesel(column_name = "login")]
    pub login: String,
    #[diesel(column_name = "username")]
    pub username: String,
    #[diesel(column_name = "secret")]
    pub secret: String,
    #[diesel(column_name = "bio")]
    pub bio: String,
    #[diesel(column_name = "avatar_url")]
    pub avatar_url: String,
}

#[derive(Queryable, Insertable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct UserEntityCreate {
    #[diesel(column_name = "login")]
    pub login: String,
    #[diesel(column_name = "username")]
    pub username: String,
    #[diesel(column_name = "secret")]
    pub secret: String,
    #[diesel(column_name = "bio")]
    pub bio: String,
    #[diesel(column_name = "avatar_url")]
    pub avatar_url: String,
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
