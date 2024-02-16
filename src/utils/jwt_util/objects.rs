use crate::data::repository::user::objects::UserDataResponse;

pub struct JwtObject {
    pub uuid: String,
    pub username: String,
}

pub struct JwtResult {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone)]
pub struct JwtDecoderResult {
    pub uuid: String,
    pub username: String,
}

pub trait JwtMapper {
    fn map(&self) -> JwtObject;
}

impl JwtMapper for UserDataResponse {
    fn map(&self) -> JwtObject {
        JwtObject {
            uuid: self.id.to_string().clone(),
            username: self.username.clone(),
        }
    }
}

impl JwtMapper for JwtDecoderResult {
    fn map(&self) -> JwtObject {
        JwtObject {
            uuid: self.uuid.clone(),
            username: self.username.clone(),
        }
    }
}
