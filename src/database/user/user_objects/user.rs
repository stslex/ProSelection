use uuid::Uuid;

#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct User {
    pub id: Uuid,
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
