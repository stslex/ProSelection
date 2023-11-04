use serde::Serialize;

pub enum LoginError {
    NotFound,
    Other,
}

#[derive(Serialize)]
pub struct LoginOk {
    pub uuid: String,
    pub username: String,
    pub token: String,
}
