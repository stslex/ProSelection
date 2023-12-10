use serde::Serialize;

pub enum LoginError {
    NotFound,
    Other,
}

#[derive(Serialize)]
pub struct LoginOk {
    pub uuid: String,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
}
