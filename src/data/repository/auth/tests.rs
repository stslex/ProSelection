#[cfg(test)]
mod tests {

    use crate::data::{
        database::tests::database_test_utls::run_migration_get_conn,
        repository::auth::{objects::RegistrationData, AuthRepository},
    };
    use std::env;
    use tokio_test::assert_ok;

    #[tokio::test]
    async fn test_registration_valid_data() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let connection = run_migration_get_conn().await.unwrap();

        let expected_username = "test_username";
        let data = RegistrationData {
            login: "test_login",
            username: expected_username,
            password: "test_password",
        };

        let outcome = connection.registration(&data).await;

        println!("result: {:?}", outcome);
        let is_valid = match outcome {
            Result::Ok(res) => {
                res.username == expected_username
                    && res.uuid != ""
                    && res.access_token != ""
                    && res.refresh_token != ""
            }
            _ => false,
        };
        assert!(is_valid)
    }

    #[tokio::test]
    async fn test_verify_token_valid_token() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let connection = run_migration_get_conn().await.unwrap();

        let expected_username = "test_username";
        let data = RegistrationData {
            login: "test_login",
            username: expected_username,
            password: "test_password",
        };

        let reg_outcome = connection.registration(&data).await.unwrap();
        let outcome = connection
            .verify_token(&reg_outcome.uuid, &reg_outcome.username)
            .await
            .unwrap();

        let is_valid = outcome.username == expected_username;
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_verify_token_invalid_token() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let connection = run_migration_get_conn().await.unwrap();

        let username = "test_username";
        let uuid = uuid::Uuid::new_v4().to_string();

        let outcome = connection.verify_token(&uuid, username).await;
        println!("result: {:?}", outcome);

        assert!(outcome.is_err())
    }

    #[tokio::test]
    async fn test_login_valid_credentials() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let connection = run_migration_get_conn().await.unwrap();

        let expected_login = "test_login";
        let expected_password = "test_password";
        let data = RegistrationData {
            login: expected_login,
            username: "test_username",
            password: expected_password,
        };

        let res = connection.registration(&data).await;
        assert_ok!(res);

        let outcome = connection.login(expected_login, expected_password).await;
        println!("result: {:?}", outcome);

        let is_valid = outcome.is_ok();

        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_login_invalid_credentials() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let connection = run_migration_get_conn().await.unwrap();

        let login = "test_login";
        let password = "invalid_password";

        let outcome = connection.login(login, password).await;

        let is_valid = outcome.is_err();
        assert!(is_valid);
    }
}
