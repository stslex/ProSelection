use uuid::Uuid;

#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub login: String,
    pub username: String,
    pub secret: String,
}
