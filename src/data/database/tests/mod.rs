#[cfg(test)]
pub mod database_test_utls {

    use crate::Conn;
    use diesel::Connection;

    #[cfg(test)]
    pub async fn get_test_conn() -> Conn {
        use std::collections::HashMap;

        use rocket::figment::value::Value;

        let url: &str = "postgres://postgres:postgres@localhost:5432/postgres";
        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();
        database_config.insert("url", Value::from(url));
        databases.insert("diesel_postgres_pool", database_config);

        let figment = rocket::Config::figment().merge(("databases", databases));
        let rocket = rocket::custom(figment)
            .attach(Conn::fairing())
            .ignite()
            .await
            .expect("unable to create rocket instance");
        Conn::get_one(&rocket)
            .await
            .expect("unable to get db connection")
    }

    #[cfg(test)]
    pub async fn run_migration_get_conn() -> Result<Conn, String> {
        use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

        const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
        let connection = get_test_conn().await;
        connection
            .run(move |db| {
                let _ = db.begin_test_transaction();
                match db.run_pending_migrations(MIGRATIONS) {
                    Ok(m_version) => {
                        println!("Database migrations ran successfully: {:?}", m_version);
                        Result::Ok(())
                    }
                    Err(e) => {
                        println!("Failed to run database migrations: {:?}", e);
                        Result::Err("Failed to run database migrations".to_string())
                    }
                }
            })
            .await
            .map(|_| connection)
    }
}

#[cfg(test)]
mod test_db_transition {

    use diesel::{result::Error, Connection};

    use crate::data::database::tests::database_test_utls::get_test_conn;

    #[tokio::test]
    async fn test_db_conn() {
        let connection = get_test_conn().await;
        let result = connection
            .run(|con| {
                con.test_transaction::<Result<String, Error>, Error, _>(|_| {
                    Ok(Ok("test".to_string()))
                })
            })
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[tokio::test]
    async fn test_db_migration() {
        let result =
            crate::data::database::tests::database_test_utls::run_migration_get_conn().await;
        assert!(result.is_ok());
    }
}
