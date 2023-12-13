#[cfg(test)]
pub mod database_test_utls {

    use diesel::{Connection, ConnectionError, PgConnection};
    use log::error;
    use std::fmt::Debug;

    #[cfg(test)]
    pub fn process_test_transition<T, E, F>(transition: F) -> T
    where
        F: FnOnce() -> Result<T, E>,
        E: Debug,
    {
        dotenv::dotenv().ok();
        let database_url = "postgres://postgres:postgres@localhost:5432/postgres";
        let connection = establish_connection(&database_url).unwrap();
        connection.test_transaction::<T, E, F>(transition)
    }

    #[cfg(test)]
    fn establish_connection(database_url: &str) -> Result<PgConnection, ConnectionError> {
        match PgConnection::establish(&database_url) {
            Ok(value) => Ok(value),
            Err(e) => {
                error!("Could not connect to PostgreSQL.");
                error!("Error connecting to {}", database_url);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod test_db {
    use diesel::result::Error;

    use super::database_test_utls::process_test_transition;

    #[test]
    fn test_db_transition() {
        let result = process_test_transition::<Result<_, Error>, Error, _>(|| Ok(Ok("test")));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_db_transition_error() {
        let result = process_test_transition::<Result<String, _>, Error, _>(|| {
            Ok(Err(Error::RollbackTransaction))
        });
        assert_eq!(result.err().unwrap(), Error::RollbackTransaction);
    }
}
