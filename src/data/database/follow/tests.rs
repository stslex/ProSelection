#[cfg(test)]
mod tests {

    use crate::data::database::{
        follow::{objects::FollowEntityCreate, FollowDatabase},
        tests::database_test_utls::run_migration_get_conn,
    };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_follow() {
        let connection = run_migration_get_conn().await.unwrap();

        let current_uuid = Uuid::new_v4();

        let empty_result = connection
            .get_followers_count(&current_uuid.to_string())
            .await;
        assert!(empty_result.is_ok());
        assert!(empty_result.unwrap().eq(&0));
    }

    #[tokio::test]
    async fn test_follow() {
        let connection = run_migration_get_conn().await.unwrap();

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

    #[tokio::test]
    async fn test_followers() {
        let connection = run_migration_get_conn().await.unwrap();

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

        let followers_get_result = connection
            .get_followers_count(&followed_uuid.to_string())
            .await;

        assert!(followers_get_result.is_ok());

        let follow_get_result = followers_get_result.unwrap();
        eprintln!("new count {:?}", follow_get_result);

        assert!(follow_get_result.eq(&(current_count + 1)));
    }
}
