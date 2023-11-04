use uuid::Uuid;

use crate::database::user::user_objects::user::User;

pub struct JwtObject {
    pub uuid: Uuid,
    pub username: String,
}

pub trait JwtMapper {
    fn map(&self) -> JwtObject;
}

impl JwtMapper for User {
    fn map(&self) -> JwtObject {
        JwtObject {
            uuid: self.id,
            username: self.username.clone(),
        }
    }
}

pub trait JwtUtil {
    fn generate(&self) -> &'static str;
}

impl JwtUtil for JwtObject {
    fn generate(&self) -> &'static str {
        // todo!("generate jwt");
        "token"
    }
}
