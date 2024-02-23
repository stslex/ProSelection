#[cfg(test)]
mod test_decoder {

    use std::{collections::BTreeMap, env};

    use hmac::{digest::KeyInit, Hmac};
    use jwt::SignWithKey;
    use sha2::Sha256;

    use crate::presenter::routes::auth::validators::jwt_decoder::{
        objects::JwtDecoderError, JwtDecoder,
    };

    const EXPECTED_UUID: &str = "expected_uuid";
    const EXPECTED_USERNAME: &str = "expected_username";
    const SECRET_TEST: &str = "secret_test";

    #[test]
    fn test_decode_access() {
        // Arrange
        env::set_var("JWT_ACCESS_SECRET", SECRET_TEST);
        let exp_time = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(1))
            .expect("Failed to add days")
            .timestamp();
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(SECRET_TEST.as_bytes()).expect("Failed to create key");
        let mut claims = BTreeMap::new();
        claims.insert("uuid", EXPECTED_UUID.to_string());
        claims.insert("username", EXPECTED_USERNAME.to_string());
        claims.insert("exp_time", exp_time.to_string());

        let binding = claims.sign_with_key(&key).ok().unwrap().to_owned();
        let jwt = binding.as_str();

        // Act
        let result = jwt.decode_access();

        // Assert
        assert!(result.is_ok());
        let decoder_result = result.unwrap();
        assert_eq!(decoder_result.uuid, EXPECTED_UUID);
        assert_eq!(decoder_result.username, EXPECTED_USERNAME);
    }

    #[test]
    fn test_decode_refresh() {
        // Arrange
        env::set_var("JWT_REFRESH_SECRET", SECRET_TEST);
        let exp_time = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(1))
            .expect("Failed to add days")
            .timestamp();
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(SECRET_TEST.as_bytes()).expect("Failed to create key");
        let mut claims = BTreeMap::new();
        claims.insert("uuid", EXPECTED_UUID.to_string());
        claims.insert("username", EXPECTED_USERNAME.to_string());
        claims.insert("exp_time", exp_time.to_string());

        let binding = claims.sign_with_key(&key).ok().unwrap().to_owned();
        let jwt = binding.as_str();

        // Act
        let result = jwt.decode_refresh();

        // Assert
        assert!(result.is_ok());
        let decoder_result = result.unwrap();
        assert_eq!(decoder_result.uuid, EXPECTED_UUID);
        assert_eq!(decoder_result.username, EXPECTED_USERNAME);
    }

    #[test]
    fn test_decode() {
        // Arrange
        env::set_var("JWT_ACCESS_SECRET", SECRET_TEST);
        let exp_time = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(1))
            .expect("Failed to add days")
            .timestamp();
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(SECRET_TEST.as_bytes()).expect("Failed to create key");
        let mut claims = BTreeMap::new();
        claims.insert("uuid", EXPECTED_UUID.to_string());
        claims.insert("username", EXPECTED_USERNAME.to_string());
        claims.insert("exp_time", exp_time.to_string());

        let binding = claims.sign_with_key(&key).ok().unwrap().to_owned();
        let jwt = binding.as_str();

        // Act
        let result = jwt.decode(SECRET_TEST.as_bytes());

        // Assert
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.uuid, EXPECTED_UUID);
        assert_eq!(result.username, EXPECTED_USERNAME);
    }

    #[test]
    fn test_decode_expired_token() {
        // Arrange
        env::set_var("JWT_ACCESS_SECRET", SECRET_TEST);
        let exp_time = 0;
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(SECRET_TEST.as_bytes()).expect("Failed to create key");
        let mut claims = BTreeMap::new();
        claims.insert("uuid", EXPECTED_UUID.to_string());
        claims.insert("username", EXPECTED_USERNAME.to_string());
        claims.insert("exp_time", exp_time.to_string());

        let binding = claims.sign_with_key(&key).ok().unwrap().to_owned();
        let jwt = binding.as_str();

        // Act
        let result = jwt.decode(SECRET_TEST.as_bytes());

        // Assert
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), JwtDecoderError::ExpiredSignature);
    }
}
