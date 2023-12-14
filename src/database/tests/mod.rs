#[cfg(test)]
pub mod database_test_utls {

    use crate::database::Conn;
    use diesel::r2d2;
    use diesel::r2d2::ConnectionManager;
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
    pub fn get_test_conn() -> Conn {
        let url = "postgres://postgres:postgres@localhost:5432/postgres";
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create database pool");
        let connection = pool.get().expect("Failed to get database connection");
        Conn(connection)
    }
}

#[cfg(test)]
mod test_db_transition {
    use diesel::{result::Error, Connection};

    use crate::database::tests::database_test_utls::{establish_connection, get_test_conn};

    #[test]
    fn test_db_transition() {
        let connection = establish_connection().unwrap();
        let result = connection.test_transaction::<Result<_, Error>, Error, _>(|| Ok(Ok("test")));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_db_conn() {
        let connection = get_test_conn();
        let result = connection
            .test_transaction::<Result<String, Error>, Error, _>(|| Ok(Ok("test".to_owned())));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_db_transition_error() {
        let connection = establish_connection().unwrap();
        let result = connection.test_transaction::<Result<String, _>, Error, _>(|| {
            Ok(Err(Error::RollbackTransaction))
        });
        assert_eq!(result.err().unwrap(), Error::RollbackTransaction);
    }
}
