#[cfg(test)]
mod tests {

    use diesel::Connection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use uuid::Uuid;

    use crate::data::{
        database::{
            tests::database_test_utls::get_test_conn,
            user::{objects::UserEntityCreate, UserDatabase},
        },
        repository::user::objects::UserSearchDataRequest,
    };
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    #[tokio::test]
    async fn test_user_insert() {
        let connection = get_test_conn().await;

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        let check_login = "login";
        let check_username = "username";
        let check_secret = "secret";
        let check_avatar_url = "avatar_url";
        let check_bio = "bio";
        let check_user = UserEntityCreate {
            login: check_login.to_owned(),
            username: check_username.to_owned(),
            secret: check_secret.to_owned(),
            avatar_url: check_avatar_url.to_owned(),
            bio: check_bio.to_owned(),
        };
        println!("checked_user: {:?}", check_user);

        let insert_user_result = connection.insert_user(check_user).await;

        assert!(insert_user_result.is_ok());

        let inserted_user = insert_user_result.unwrap();
        println!("inserted_user: {:?}", inserted_user);

        assert_eq!(inserted_user.login, check_login.to_owned());
        assert_eq!(inserted_user.username, check_username.to_owned());
        assert_eq!(inserted_user.secret, check_secret.to_owned());
        assert_eq!(inserted_user.avatar_url, check_avatar_url.to_owned());
        assert_eq!(inserted_user.bio, check_bio.to_owned());
    }

    #[tokio::test]
    async fn test_user_get() {
        let connection = get_test_conn().await;

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        let check_login = "login";
        let check_username = "username";
        let check_secret = "secret";
        let check_avatar_url = "avatar_url";
        let check_bio = "bio";
        let check_user = UserEntityCreate {
            login: check_login.to_owned(),
            username: check_username.to_owned(),
            secret: check_secret.to_owned(),
            avatar_url: check_avatar_url.to_owned(),
            bio: check_bio.to_owned(),
        };

        let insert_user_result = connection.insert_user(check_user).await;
        assert!(insert_user_result.is_ok());
        let inserted_user = insert_user_result.unwrap();

        let get_user_result = connection.get_user(&inserted_user.id.to_string()).await;
        assert!(get_user_result.is_ok());

        let get_user = get_user_result.unwrap();
        assert_eq!(get_user.login, check_login.to_owned());
        assert_eq!(get_user.username, check_username.to_owned());
        assert_eq!(get_user.secret, check_secret.to_owned());
        assert_eq!(get_user.avatar_url, check_avatar_url.to_owned());
        assert_eq!(get_user.bio, check_bio.to_owned());
    }

    #[tokio::test]
    async fn test_user_get_login() {
        let connection = get_test_conn().await;

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        let check_login = "login";
        let check_username = "username";
        let check_secret = "secret";
        let check_avatar_url = "avatar_url";
        let check_bio = "bio";
        let check_user = UserEntityCreate {
            login: check_login.to_owned(),
            username: check_username.to_owned(),
            secret: check_secret.to_owned(),
            avatar_url: check_avatar_url.to_owned(),
            bio: check_bio.to_owned(),
        };

        let insert_user_result = connection.insert_user(check_user).await;
        assert!(insert_user_result.is_ok());

        let get_user_by_login_result = connection.get_user_by_login(check_login).await;
        assert!(get_user_by_login_result.is_ok());

        let get_user_by_login = get_user_by_login_result.unwrap();
        assert_eq!(get_user_by_login.login, check_login.to_owned());
        assert_eq!(get_user_by_login.username, check_username.to_owned());
        assert_eq!(get_user_by_login.secret, check_secret.to_owned());
        assert_eq!(get_user_by_login.avatar_url, check_avatar_url.to_owned());
        assert_eq!(get_user_by_login.bio, check_bio.to_owned());
    }

    #[tokio::test]
    async fn test_user_get_username() {
        let connection = get_test_conn().await;

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        let check_login = "login";
        let check_username = "username";
        let check_secret = "secret";
        let check_avatar_url = "avatar_url";
        let check_bio = "bio";
        let check_user = UserEntityCreate {
            login: check_login.to_owned(),
            username: check_username.to_owned(),
            secret: check_secret.to_owned(),
            avatar_url: check_avatar_url.to_owned(),
            bio: check_bio.to_owned(),
        };

        let insert_user_result = connection.insert_user(check_user).await;
        assert!(insert_user_result.is_ok());

        let get_user_by_username_result = connection.get_user_by_username(check_username).await;
        assert!(get_user_by_username_result.is_ok());

        let get_user_by_username = get_user_by_username_result.unwrap();
        assert_eq!(get_user_by_username.login, check_login.to_owned());
        assert_eq!(get_user_by_username.username, check_username.to_owned());
        assert_eq!(get_user_by_username.secret, check_secret.to_owned());
        assert_eq!(get_user_by_username.avatar_url, check_avatar_url.to_owned());
        assert_eq!(get_user_by_username.bio, check_bio.to_owned());
    }

    #[tokio::test]
    async fn test_user_get_search() {
        let connection = get_test_conn().await;

        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                let _ = db.run_pending_migrations(MIGRATIONS);
            })
            .await;

        let check_login = "login";
        let check_username = "username";
        let check_secret = "secret";
        let check_avatar_url = "avatar_url";
        let check_bio = "bio";
        let check_user = UserEntityCreate {
            login: check_login.to_owned(),
            username: check_username.to_owned(),
            secret: check_secret.to_owned(),
            avatar_url: check_avatar_url.to_owned(),
            bio: check_bio.to_owned(),
        };

        let insert_user_result = connection.insert_user(check_user).await;
        assert!(insert_user_result.is_ok());

        let current_user_uuid = Uuid::new_v4().to_string();
        let user_search_request = UserSearchDataRequest {
            query: check_username,
            uuid: current_user_uuid.as_str(),
            page: 1,
            page_size: 15,
        };

        let get_user_search_result = connection.search_users(&user_search_request).await;
        assert!(get_user_search_result.is_ok());

        let get_user_search = get_user_search_result.unwrap();
        assert_eq!(get_user_search.len(), 1);

        let searched_user = get_user_search.get(0).unwrap();
        assert_eq!(searched_user.login, check_login.to_owned());
        assert_eq!(searched_user.username, check_username.to_owned());
        assert_eq!(searched_user.secret, check_secret.to_owned());
        assert_eq!(searched_user.avatar_url, check_avatar_url.to_owned());
        assert_eq!(searched_user.bio, check_bio.to_owned());
    }
}
