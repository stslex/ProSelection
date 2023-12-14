#[cfg(test)]
mod tests {
    use crate::{
        database::user::user_objects::user::User,
        utils::jwt_util::objects::{JwtDecoderResult, JwtMapper},
    };
    use uuid::Uuid;

    #[test]
    fn test_jwt_mapper_for_user() {
        let user = User {
            id: Uuid::new_v4(),
            username: "john_doe".to_owned(),
            login: "login".to_owned(),
            secret: "smth_secret".to_owned(),
        };

        let jwt_mapper = user.map();
        assert_eq!(jwt_mapper.uuid, user.id.to_string());
        assert_eq!(jwt_mapper.username, user.username);
    }

    #[test]
    fn test_jwt_mapper_for_jwt_decoder_result() {
        let expect_res = JwtDecoderResult {
            uuid: "456".to_owned(),
            username: "jane_smith".to_owned(),
        };

        let jwt_mapper = expect_res.map();
        assert_eq!(jwt_mapper.uuid, expect_res.uuid);
        assert_eq!(jwt_mapper.username, expect_res.username);
    }
}
