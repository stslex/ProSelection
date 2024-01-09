use std::env;

use std::collections::HashMap;

use dotenv::dotenv;
use rocket::figment::{value::Value, Figment};

pub fn from_env() -> Figment {
    dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let address = env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    let database_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");

    database_config.insert("url", Value::from(database_url));

    databases.insert("diesel_postgres_pool", database_config);

    Figment::from(rocket::Config::default())
        .merge(("address", address))
        .merge(("port", port))
        .merge(("databases", databases))
}
