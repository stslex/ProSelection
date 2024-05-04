#[cfg(test)]
mod tests {

    use crate::data::{
        database::tests::database_test_utls::run_migration_get_conn,
        repository::{favourite::FavouriteRepository, objects::PagingDomainRequest},
    };
    use tokio_test::assert_ok;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_favourites_count() {
        let connection = run_migration_get_conn().await.unwrap();

        let uuid = Uuid::new_v4().to_string();
        let count_empty_result = connection.get_favourites_count(&uuid).await;
        assert!(count_empty_result.is_ok());

        let count_empty = count_empty_result.unwrap();
        assert_eq!(count_empty, 0);
    }

    #[tokio::test]
    async fn test_add_favourites() {
        let connection = run_migration_get_conn().await.unwrap();

        let uuid = Uuid::new_v4().to_string();
        let favourite_uuid = Uuid::new_v4().to_string();
        let title = "Favourite Title";

        let result = connection
            .add_favourite(&uuid, &favourite_uuid, title)
            .await;
        assert_ok!(result);
    }

    #[tokio::test]
    async fn test_remove_favourites() {
        let connection = run_migration_get_conn().await.unwrap();

        let uuid = Uuid::new_v4().to_string();
        let favourite_uuid = Uuid::new_v4().to_string();
        let title = "Favourite Title";

        let add_favourite = connection
            .add_favourite(&uuid, &favourite_uuid, title)
            .await;
        assert_ok!(add_favourite);

        let result = connection.remove_favourite(&uuid, &favourite_uuid).await;
        assert_ok!(result);
    }

    #[tokio::test]
    async fn test_get_favourites() {
        let connection = run_migration_get_conn().await.unwrap();

        let user_uuid_check = Uuid::new_v4();
        let favourite_uuid_check = Uuid::new_v4();

        let uuid = user_uuid_check.to_string();
        let favourite_uuid = favourite_uuid_check.to_string();

        let title = "Favourite Title";

        let add_favourite_result = connection
            .add_favourite(&uuid, &favourite_uuid, title)
            .await;
        assert_ok!(add_favourite_result);

        let paging_domain_request = PagingDomainRequest {
            user_uuid: &uuid,
            request_uuid: &uuid,
            query: title,
            page: 1,
            page_size: 15,
        };

        let get_favourite_result = connection.get_user_favourites(&paging_domain_request).await;
        assert!(get_favourite_result.is_ok());

        let favourites = get_favourite_result.to_owned().unwrap();
        let items = favourites.result;
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, title);
        assert_eq!(items[0].user_uuid, user_uuid_check);
        assert_eq!(items[0].favourite_uuid, favourite_uuid_check);

        assert_eq!(favourites.total, 1);
        assert_eq!(favourites.page, paging_domain_request.page);
        assert_eq!(favourites.page_size, paging_domain_request.page_size);
        assert_eq!(favourites.has_more, false);
    }
}
