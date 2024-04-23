#[cfg(test)]
mod tests {

    use crate::data::{
        database::tests::database_test_utls::get_test_conn,
        repository::{
            auth::{objects::RegistrationData, AuthRepository},
            follow::FollowRepository,
        },
    };
    use diesel::Connection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use std::env;

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    #[tokio::test]
    async fn test_get_follower_count() {
        let connection = get_test_conn().await;

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        // create follower and followed users
        // set tokens
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");

        let first_user_data = RegistrationData {
            login: "test_login_first",
            username: "username_first",
            password: "test_passwd_first",
        };

        let second_user_data = RegistrationData {
            login: "test_login_second",
            username: "username_second",
            password: "test_passwd_second",
        };

        let outcome_first = connection.registration(&first_user_data).await;
        assert!(outcome_first.is_ok());

        let outcome_second = connection.registration(&second_user_data).await;
        assert!(outcome_second.is_ok());

        let follower_uuid = outcome_first.unwrap().uuid;
        let followed_uuid = outcome_second.unwrap().uuid;

        // check if follower has no followers
        let count_empty_result = connection.get_followers_count(&follower_uuid).await;
        assert!(count_empty_result.is_ok());

        let count_empty = count_empty_result.unwrap();
        assert_eq!(count_empty, 0);

        let follow_result = connection.follow_user(&follower_uuid, &followed_uuid).await;
        assert!(follow_result.is_ok());

        let count_result = connection.get_followers_count(&followed_uuid).await;
        assert!(count_result.is_ok());
        let count = count_result.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_get_following_count() {
        let connection = get_test_conn().await;

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        // create follower and followed users
        // set tokens
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");

        let first_user_data = RegistrationData {
            login: "test_login_first",
            username: "username_first",
            password: "test_passwd_first",
        };

        let second_user_data = RegistrationData {
            login: "test_login_second",
            username: "username_second",
            password: "test_passwd_second",
        };

        let outcome_first = connection.registration(&first_user_data).await;
        assert!(outcome_first.is_ok());

        let outcome_second = connection.registration(&second_user_data).await;
        assert!(outcome_second.is_ok());

        let follower_uuid = outcome_first.unwrap().uuid;
        let followed_uuid = outcome_second.unwrap().uuid;

        // check if follower has no followers
        let count_empty_result = connection.get_following_count(&follower_uuid).await;
        assert!(count_empty_result.is_ok());

        let count_empty = count_empty_result.unwrap();
        assert_eq!(count_empty, 0);

        let follow_result = connection.follow_user(&follower_uuid, &followed_uuid).await;
        assert!(follow_result.is_ok());

        let count_result = connection.get_following_count(&follower_uuid).await;
        assert!(count_result.is_ok());
        let count = count_result.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_combine_state() {
        let connection = get_test_conn().await;

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        // create follower and followed users
        // set tokens
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");

        let first_user_data = RegistrationData {
            login: "test_login_first",
            username: "username_first",
            password: "test_passwd_first",
        };

        let second_user_data = RegistrationData {
            login: "test_login_second",
            username: "username_second",
            password: "test_passwd_second",
        };

        let outcome_first = connection.registration(&first_user_data).await;
        assert!(outcome_first.is_ok());

        let outcome_second = connection.registration(&second_user_data).await;
        assert!(outcome_second.is_ok());

        let follower_uuid = outcome_first.unwrap().uuid;
        let followed_uuid = outcome_second.unwrap().uuid;

        let follow_result = connection.follow_user(&follower_uuid, &followed_uuid).await;
        assert!(follow_result.is_ok());

        let count_result = connection.get_following_count(&follower_uuid).await;
        assert!(count_result.is_ok());
        let count = count_result.unwrap();
        assert_eq!(count, 1);

        let is_follow = connection
            .is_following(&follower_uuid, &followed_uuid)
            .await;
        assert!(is_follow.is_ok());
        assert!(is_follow.unwrap());

        let unfollow_result = connection
            .un_follow_user(&follower_uuid, &followed_uuid)
            .await;
        assert!(unfollow_result.is_ok());

        let count_result = connection.get_following_count(&follower_uuid).await;
        assert!(count_result.is_ok());
        let count = count_result.unwrap();
        assert_eq!(count, 0);

        let is_follow = connection
            .is_following(&follower_uuid, &followed_uuid)
            .await;
        assert!(is_follow.is_ok());
        assert!(!is_follow.unwrap());
    }
}
