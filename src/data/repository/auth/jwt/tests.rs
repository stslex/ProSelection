#[cfg(test)]
mod test_generator {
    use std::{collections::BTreeMap, env};

    use hmac::{digest::KeyInit, Hmac};
    use jwt::{Error, Header, Token, VerifyWithKey};
    use sha2::Sha256;

    use crate::data::repository::jwt::{
        generator::JwtGeneratorError, objects::JwtObject, JwtGenerator,
    };

    const EXPECTED_UUID: &str = "expected_uuid";
    const EXPECTED_USERNAME: &str = "expected_username";
    const SECRET_TEST: &str = "secret_test";

    #[tokio::test]
    async fn test_generate_access() {
        // Arrange
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(SECRET_TEST.as_bytes()).expect("Failed to create key");
        let test_jwt_object = test_jwt_object();
        env::set_var("JWT_ACCESS_SECRET", SECRET_TEST);

        // Act
        let result = test_jwt_object.generate_access().await;

        // Assert
        assert!(result.is_ok());
        let jwt = result.unwrap();

        let token_result: Result<Token<Header, BTreeMap<String, String>, jwt::Verified>, Error> =
            jwt.verify_with_key(&key);
        assert!(token_result.is_ok());
        let binding = token_result.unwrap();
        let claims = binding.claims();
        assert_eq!(claims.get("uuid").unwrap(), EXPECTED_UUID);
        assert_eq!(claims.get("username").unwrap(), EXPECTED_USERNAME);
    }

    #[tokio::test]
    async fn test_generate_refresh() {
        // Arrange
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(SECRET_TEST.as_bytes()).expect("Failed to create key");
        let test_jwt_object = test_jwt_object();
        env::set_var("JWT_REFRESH_SECRET", SECRET_TEST);

        // Act
        let result = test_jwt_object.generate_refresh().await;

        // Assert
        assert!(result.is_ok());
        let jwt = result.unwrap();

        let token_result: Result<Token<Header, BTreeMap<String, String>, jwt::Verified>, Error> =
            jwt.verify_with_key(&key);
        assert!(token_result.is_ok());
        let binding = token_result.unwrap();
        let claims = binding.claims();
        assert_eq!(claims.get("uuid").unwrap(), EXPECTED_UUID);
        assert_eq!(claims.get("username").unwrap(), EXPECTED_USERNAME);
    }

    #[tokio::test]
    async fn test_generate_token() {
        // Arrange
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(SECRET_TEST.as_bytes()).expect("Failed to create key");
        let test_jwt_object = test_jwt_object();
        let exp_days = 3;
        env::set_var("JWT_REFRESH_SECRET", SECRET_TEST);

        // Act
        let result = test_jwt_object
            .generate_token(SECRET_TEST.as_bytes(), exp_days)
            .await;

        // Assert
        assert!(result.is_ok());
        let jwt = result.unwrap();

        let token_result: Result<Token<Header, BTreeMap<String, String>, jwt::Verified>, Error> =
            jwt.verify_with_key(&key);
        assert!(token_result.is_ok());
        let binding = token_result.unwrap();
        let claims = binding.claims();
        assert_eq!(claims.get("uuid").unwrap(), EXPECTED_UUID);
        assert_eq!(claims.get("username").unwrap(), EXPECTED_USERNAME);
    }

    #[tokio::test]
    async fn test_generate_token_out_of_rage() {
        // Arrange
        let test_jwt_object = test_jwt_object();
        let exp_days = i64::MAX;
        env::set_var("JWT_REFRESH_SECRET", SECRET_TEST);

        // Act
        let result = test_jwt_object
            .generate_token(SECRET_TEST.as_bytes(), exp_days)
            .await;
        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), JwtGeneratorError::DurationOutOfBound);
    }

    #[tokio::test]
    async fn test_generate() {
        // Arrange
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(SECRET_TEST.as_bytes()).expect("Failed to create key");
        let test_jwt_object = test_jwt_object();

        env::set_var("JWT_REFRESH_SECRET", SECRET_TEST);
        env::set_var("JWT_ACCESS_SECRET", SECRET_TEST);

        // Act
        let result = test_jwt_object.generate().await;

        // Assert
        assert!(result.is_ok());
        let jwt_result = result.unwrap();

        let token_refresh_result: Result<
            Token<Header, BTreeMap<String, String>, jwt::Verified>,
            Error,
        > = jwt_result.refresh_token.verify_with_key(&key);
        assert!(token_refresh_result.is_ok());
        let binding = token_refresh_result.unwrap();
        let claims = binding.claims();
        assert_eq!(claims.get("uuid").unwrap(), EXPECTED_UUID);
        assert_eq!(claims.get("username").unwrap(), EXPECTED_USERNAME);

        let token_access_result: Result<
            Token<Header, BTreeMap<String, String>, jwt::Verified>,
            Error,
        > = jwt_result.access_token.verify_with_key(&key);
        assert!(token_access_result.is_ok());
        let binding = token_access_result.unwrap();
        let claims = binding.claims();
        assert_eq!(claims.get("uuid").unwrap(), EXPECTED_UUID);
        assert_eq!(claims.get("username").unwrap(), EXPECTED_USERNAME);
    }

    fn test_jwt_object() -> JwtObject {
        JwtObject {
            uuid: EXPECTED_UUID.to_string(),
            username: EXPECTED_USERNAME.to_string(),
        }
    }

    #[tokio::test]
    async fn test_jwt_mapper_for_user() {
        let user = UserDataResponse {
            id: Uuid::new_v4(),
            username: "john_doe".to_owned(),
            login: "login".to_owned(),
            secret: "smth_secret".to_owned(),
            avatar_url: "avatar_url".to_owned(),
            bio: "bio".to_owned(),
        };

        let jwt_mapper = user.map().await;
        assert_eq!(jwt_mapper.uuid, user.id.to_string());
        assert_eq!(jwt_mapper.username, user.username);
    }
}
