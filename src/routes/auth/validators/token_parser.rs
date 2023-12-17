use rocket::Request;

use super::TokenParser;

const AUTH_HEADER: &str = "Authorization";
const BEARER_PREFIX: &str = "Bearer ";

impl<'r> TokenParser for Request<'r> {
    fn get_token(&self) -> Option<String> {
        let token = self.headers().get_one(AUTH_HEADER);
        match token {
            Some(token) => {
                let replaced_token = token.replace(BEARER_PREFIX, "");
                Some(replaced_token)
            }
            None => None,
        }
    }
}
