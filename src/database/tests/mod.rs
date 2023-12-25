#[cfg(test)]
pub mod database_test_utls {

    use crate::database::Conn;
    use diesel::{Connection, ConnectionError, PgConnection};
    use log::error;

    #[cfg(test)]
    pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
        let database_url = "postgres://postgres:postgres@localhost:5432/postgres";
        match PgConnection::establish(&database_url) {
            Ok(value) => Ok(value),
            Err(e) => {
                error!("Could not connect to PostgreSQL.");
                error!("Error connecting to {}", database_url);
                Err(e)
            }
        }
    }

    #[cfg(test)]
    pub async fn get_test_conn() -> Conn {
        let url: &str = "postgres://postgres:postgres@localhost:5432/postgres";
        let figment = rocket::Config::figment().merge(("databases.db.url", url));
        let rocket = rocket::custom(figment)
            .attach(Conn::fairing())
            .ignite()
            .await
            .expect("unable to create rocket instance");
        Conn::get_one(&rocket)
            .await
            .expect("unable to get db connection")
    }
}

#[cfg(test)]
mod test_db_transition {

    use diesel::{result::Error, Connection};

    use crate::database::tests::database_test_utls::{establish_connection, get_test_conn};

    #[test]
    fn test_db_transition() {
        let mut connection = establish_connection().unwrap();
        let result =
            connection.test_transaction::<Result<_, Error>, Error, _>(|con| Ok(Ok("test")));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[tokio::test]
    async fn test_db_conn() {
        let connection = get_test_conn().await;
        let result = connection
            .run(|con| {
                con.test_transaction::<Result<String, Error>, Error, _>(|conn| {
                    Ok(Ok("test".to_string()))
                })
            })
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_db_transition_error() {
        let mut connection = establish_connection().unwrap();
        let result = connection.test_transaction::<Result<String, Error>, Error, _>(|con| {
            Ok(Err(Error::RollbackTransaction))
        });

        assert_eq!(result.err().unwrap(), Error::RollbackTransaction);
    }
}
