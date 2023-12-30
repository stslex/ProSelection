use std::collections::HashMap;
use std::env;

use rocket::figment::value::Value;
use rocket::Config;

pub fn from_env() -> Config {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    let database_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");

    database_config.insert("url", Value::from(database_url));

    databases.insert("diesel_postgres_pool", database_config);

    let provider = rocket::Config::figment()
        .merge(("port", port))
        .merge(("databases", databases));
    Config::from(provider)
}
