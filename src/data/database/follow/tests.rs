#[cfg(test)]
mod tests {

    use crate::data::database::{
        follow::{objects::FollowEntityCreate, FollowDatabase},
        tests::database_test_utls::get_test_conn,
    };
    use diesel::Connection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use uuid::Uuid;

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    #[tokio::test]
    async fn test_get_follow() {
        let connection = get_test_conn().await;

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        let current_uuid = Uuid::new_v4();

        let empty_result = connection
            .get_followers_count(&current_uuid.to_string())
            .await;
        assert!(empty_result.is_ok());
        assert!(empty_result.unwrap().eq(&0));
    }

    #[tokio::test]
    async fn test_follow() {
        let connection = get_test_conn().await;

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        let current_uuid = Uuid::new_v4();
        let followed_uuid = Uuid::new_v4();

        let first_result = connection
            .get_followers_count(&current_uuid.to_string())
            .await;
        assert!(first_result.is_ok());

        let current_count = first_result.unwrap();
        eprintln!("current count {:?}", current_count);

        let follow_user = FollowEntityCreate {
            follower_uuid: current_uuid,
            followed_uuid: followed_uuid,
            follower_username: "follower_username".to_owned(),
            followed_username: "followed_username".to_owned(),
            follower_avatar_url: "follower_avatar_url".to_owned(),
            followed_avatar_url: "followed_avatar_url".to_owned(),
        };

        let follow_result = connection.follow_user(&follow_user).await;
        assert!(follow_result.is_ok());

        let follow_get_result = connection
            .get_following_count(&current_uuid.to_string())
            .await;

        assert!(follow_get_result.is_ok());

        let follow_get_result = follow_get_result.unwrap();
        eprintln!("new count {:?}", follow_get_result);

        assert!(follow_get_result.eq(&(current_count + 1)));
    }
}
