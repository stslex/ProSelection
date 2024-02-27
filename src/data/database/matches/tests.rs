#[cfg(test)]
mod tests {

    use diesel::Connection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use uuid::Uuid;

    use crate::data::database::{
        matches::{objects::MatchesEntityCreate, MatchesDatabase},
        tests::database_test_utls::get_test_conn,
    };

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    #[tokio::test]
    async fn test_add_matches_success() {
        let connection = get_test_conn().await;

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        // Add matches

        let mut user_generated_uuid = Vec::new();
        for _ in 0..10 {
            user_generated_uuid.push(Uuid::new_v4());
        }

        let match_create = MatchesEntityCreate {
            creator_uuid: Uuid::new_v4(),
            user_uuid: user_generated_uuid,
            title: "Title".to_string(),
            description: "Description".to_string(),
            url: "URL".to_string(),
        };
        let match_create_send = match_create.to_owned();
        let result = connection.add_match(match_create_send).await;
        assert!(result.is_ok());

        // Check matches
        let actual = result.unwrap();
        let expected = match_create.to_owned();
        assert_eq!(actual.creator_uuid, expected.creator_uuid);
        assert_eq!(actual.user_id, expected.user_uuid);
        assert_eq!(actual.title, expected.title);
        assert_eq!(actual.description, expected.description);
    }
}
