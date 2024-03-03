use std::env;

use std::collections::HashMap;

use crate::Conn;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use rocket::{
    figment::{value::Value, Figment},
    Build, Rocket,
};

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

    log::info!("Starting server from_env info");
    log::error!("Starting server from_env error");
    println!("Starting server from_env print");

    Figment::from(rocket::Config::default())
        .merge(("address", address))
        .merge(("port", port))
        .merge(("databases", databases))
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    println!("Running database migrations");
    let conn = Conn::get_one(&rocket).await.expect("database connection");
    conn.run(|conn| {
        // Run pending migrations
        match conn.run_pending_migrations(MIGRATIONS) {
            Ok(m_version) => {
                println!("Database migrations ran successfully: {:?}", m_version);
                Ok(rocket)
            }
            Err(e) => {
                println!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        }
    })
    .await
    .unwrap()
}
