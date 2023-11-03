use rocket::request::Request;
use rocket::response::Responder;
use rocket::response::Result;

pub enum LoginError {
    NotFound,
    Other,
}

pub struct LoginOk {
    pub uuid: String,
    pub username: String,
    pub token: String,
}

impl<'r> Responder<'r> for LoginOk {
    fn respond_to(self, request: &Request) -> Result<'r> {
        self.respond_to(request)
    }
}
