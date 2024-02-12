#[cfg(test)]
mod tests {

    use diesel::Connection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use uuid::Uuid;

    use crate::data::database::{
        favourite::UserFavouritesDatabase, tests::database_test_utls::get_test_conn,
    };

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    #[tokio::test]
    async fn test_add_and_remove_favourite() {
        let connection = get_test_conn().await;

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        // Add a favourite
        let uuid = Uuid::new_v4().to_string();
        let favourite_uuid = Uuid::new_v4().to_string();
        let title = "Favourite Title";
        let result = connection
            .add_favourite(&uuid, &favourite_uuid, title)
            .await;
        assert!(result.is_ok());

        // Remove the favourite
        let result = connection.remove_favourite(&uuid, &favourite_uuid).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_favourites_count() {
        let connection = get_test_conn().await;

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        // Add a favourite
        let uuid = Uuid::new_v4().to_string();
        let favourite_uuid = Uuid::new_v4().to_string();
        let title = "Favourite Title";
        let result = connection
            .add_favourite(&uuid, &favourite_uuid, title)
            .await;
        assert!(result.is_ok());

        // Get the count of favourites
        let count = connection.get_favourites_count(&uuid).await;
        assert!(count.is_ok());
        assert_eq!(count.unwrap(), 1);
    }
}
