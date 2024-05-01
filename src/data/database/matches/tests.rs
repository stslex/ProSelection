#[cfg(test)]
mod tests {

    use std::env;

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
        env::set_var("JWT_ACCESS_SECRET", "JWT_ACCESS_SECRET");
        env::set_var("JWT_REFRESH_SECRET", "JWT_REFRESH_SECRET");
        let connection = get_test_conn().await;

        // Create matches
        let mut user_generated_uuid = Vec::new();
        for _ in 0..10 {
            user_generated_uuid.push(Uuid::new_v4());
        }

        let current_time_ms = chrono::Utc::now().timestamp_millis();
        let match_create = MatchesEntityCreate {
            creator_uuid: Uuid::new_v4(),
            participants_uuid: Vec::new(),
            title: "title".to_string(),
            description: "description".to_string(),
            cover_url: "url".to_string(),
            status: "status".to_string(),
            created_at: current_time_ms,
            updated_at: current_time_ms,
            expires_at: current_time_ms,
        };
        let match_create_send = match_create.to_owned();

        // Run migrations
        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                match db.run_pending_migrations(MIGRATIONS) {
                    Ok(m_version) => {
                        println!("Database migrations ran successfully: {:?}", m_version);
                    }
                    Err(e) => {
                        println!("Failed to run database migrations: {:?}", e);
                    }
                }
            })
            .await;

        // Add matches
        let result = connection.add_match(match_create_send).await.unwrap();

        // Check matches
        let actual = result.to_owned();
        let expected = match_create.to_owned();

        println!("actual: {:?}", actual);
        println!("expected: {:?}", expected);

        let is_valid = actual.creator_uuid == expected.creator_uuid
            && actual.participants_uuid == expected.participants_uuid
            && actual.title == expected.title
            && actual.description == expected.description;

        assert!(is_valid);
    }
}
