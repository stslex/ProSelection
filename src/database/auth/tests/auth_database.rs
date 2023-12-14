#[cfg(test)]
mod tests {

    use crate::database::auth::{
        reg_objects::{RegistrationData, RegistrationFieldValidError, RegistrationOutcome},
        AuthorizationDatabase, AuthorizationOutcome, VerifyTokenOutcome,
    };
    use crate::database::tests::database_test_utls::get_test_conn;
    use diesel::Connection;
    use std::{env, fmt::Error};
    embed_migrations!("migrations");

    #[test]
    fn test_registration_valid_data() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let conn = get_test_conn();
        let _ = embedded_migrations::run(&*conn);

        let expected_username = "test_username";
        let data = RegistrationData {
            login: "test_login",
            username: expected_username,
            password: "test_password",
        };

        let outcome =
            conn.test_transaction::<RegistrationOutcome, Error, _>(|| Ok(conn.registration(data)));
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

    #[test]
    fn test_registration_invalid_data() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let conn = get_test_conn();
        let _ = embedded_migrations::run(&*conn);

        let data = RegistrationData {
            login: "test_login",
            username: "test_username",
            password: "", // invalid password
        };

        let outcome =
            conn.test_transaction::<RegistrationOutcome, Error, _>(|| Ok(conn.registration(data)));

        let is_valid = match outcome {
            RegistrationOutcome::RegistrationFieldValid(error) => match error {
                RegistrationFieldValidError::WeakPassword => true,
                _ => false,
            },
            _ => false,
        };
        assert!(is_valid);
    }

    #[test]
    fn test_verify_token_valid_token() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let conn = get_test_conn();
        let _ = embedded_migrations::run(&*conn);

        let username = "test_username";
        let data = RegistrationData {
            login: "test_login",
            username: username,
            password: "test_password",
        };

        let outcome = conn.test_transaction::<VerifyTokenOutcome, Error, _>(|| {
            let reg_outcome = conn.registration(data);
            match reg_outcome {
                RegistrationOutcome::Ok(res) => Ok(conn.verify_token(&res.uuid, &res.username)),
                _ => Ok(VerifyTokenOutcome::Other),
            }
        });

        let is_valid = match outcome {
            VerifyTokenOutcome::Ok(res) => res.username == username,
            _ => false,
        };
        assert!(is_valid);
    }

    #[test]
    fn test_verify_token_invalid_token() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let conn = get_test_conn();
        let _ = embedded_migrations::run(&*conn);

        let username = "test_username";
        let uuid = uuid::Uuid::new_v4().to_string();

        let outcome = conn.test_transaction::<VerifyTokenOutcome, Error, _>(|| {
            Ok(conn.verify_token(&uuid, username))
        });

        println!("result: {:?}", outcome);
        let is_valid = match outcome {
            VerifyTokenOutcome::NotFound => true,
            _ => false,
        };

        assert!(is_valid)
    }

    #[test]
    fn test_login_valid_credentials() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let conn = get_test_conn();

        let login = "test_login";
        let password = "test_password";
        let data = RegistrationData {
            login: login,
            username: "test_username",
            password: password,
        };

        let outcome = conn.test_transaction::<AuthorizationOutcome, Error, _>(|| {
            let _ = embedded_migrations::run(&*conn).expect("Failed to run migrations");
            conn.registration(data);
            Ok(conn.login(login, password))
        });

        println!("result: {:?}", outcome);

        let is_valid = match outcome {
            AuthorizationOutcome::Ok(_res) => true,
            _ => false,
        };

        assert!(is_valid);
    }

    #[test]
    fn test_login_invalid_credentials() {
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let conn = get_test_conn();
        let _ = embedded_migrations::run(&*conn);

        let login = "test_login";
        let password = "invalid_password";
        let outcome = conn
            .test_transaction::<AuthorizationOutcome, Error, _>(|| Ok(conn.login(login, password)));

        let is_valid = match outcome {
            AuthorizationOutcome::Ok(_) => false,
            _ => true,
        };
        assert!(is_valid);
    }
}
