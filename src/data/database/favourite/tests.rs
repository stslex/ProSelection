#[cfg(test)]
mod tests {
    use crate::data::database::{
        favourite::UserFavouritesDatabase, tests::database_test_utls::run_migration_get_conn,
    };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_add_and_remove_favourite() {
        let connection = run_migration_get_conn().await.unwrap();

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
        let connection = run_migration_get_conn().await.unwrap();

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
