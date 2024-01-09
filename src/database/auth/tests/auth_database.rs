#[cfg(test)]
mod tests {

    use crate::database::auth::{
        reg_objects::{RegistrationData, RegistrationFieldValidError, RegistrationOutcome},
        AuthorizationDatabase, AuthorizationOutcome, VerifyTokenOutcome,
    };
    use crate::database::tests::database_test_utls::get_test_conn;
    use diesel::Connection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use rocket_sync_db_pools::diesel;
    use std::{env, fmt::Error};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    #[tokio::test]
    async fn test_registration_valid_data() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let connection = get_test_conn().await;

        let expected_username = "test_username";
        let data = RegistrationData {
            login: "test_login".to_owned(),
            username: expected_username.to_owned(),
            password: "test_password".to_owned(),
        };

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        let outcome = connection.registration(data).await;

        println!("result: {:?}", outcome);
        let is_valid = match outcome {
            RegistrationOutcome::Ok(res) => {
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
    async fn test_registration_invalid_data() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let connection = get_test_conn().await;

        let data = RegistrationData {
            login: "test_login".to_owned(),
            username: "test_username".to_owned(),
            password: "".to_owned(), // invalid password
        };

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        let outcome = connection.registration(data).await;

        let is_valid = match outcome {
            RegistrationOutcome::RegistrationFieldValid(error) => match error {
                RegistrationFieldValidError::WeakPassword => true,
                _ => false,
            },
            _ => false,
        };
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_verify_token_valid_token() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let connection = get_test_conn().await;

        let username = "test_username";
        let data = RegistrationData {
            login: "test_login".to_owned(),
            username: username.to_owned(),
            password: "test_password".to_owned(),
        };

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        let reg_outcome = connection.registration(data).await;
        let outcome = match reg_outcome {
            RegistrationOutcome::Ok(res) => Result::Ok::<VerifyTokenOutcome, Error>(
                connection.verify_token(&res.uuid, &res.username).await,
            ),
            _ => Ok(VerifyTokenOutcome::Other),
        }
        .unwrap();

        let is_valid = match outcome {
            VerifyTokenOutcome::Ok(res) => res.username == username,
            _ => false,
        };
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_verify_token_invalid_token() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let connection = get_test_conn().await;

        let username = "test_username";
        let uuid = uuid::Uuid::new_v4().to_string();

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        let outcome = connection.verify_token(&uuid, username).await;

        println!("result: {:?}", outcome);
        let is_valid = match outcome {
            VerifyTokenOutcome::NotFound => true,
            _ => false,
        };

        assert!(is_valid)
    }

    #[tokio::test]
    async fn test_login_valid_credentials() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let connection = get_test_conn().await;

        let login = "test_login";
        let password = "test_password";
        let data = RegistrationData {
            login: login.to_owned(),
            username: "test_username".to_owned(),
            password: password.to_owned(),
        };

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        connection.registration(data).await;
        let outcome = connection.login(login, password).await;

        println!("result: {:?}", outcome);

        let is_valid = match outcome {
            AuthorizationOutcome::Ok(_res) => true,
            _ => false,
        };

        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_login_invalid_credentials() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let connection = get_test_conn().await;

        let login = "test_login";
        let password = "invalid_password";

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        let outcome = connection.login(login, password).await;

        let is_valid = match outcome {
            AuthorizationOutcome::Ok(_) => false,
            _ => true,
        };
        assert!(is_valid);
    }
}
