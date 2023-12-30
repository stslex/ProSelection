use rocket::Config;
use std::env;

pub fn from_env() -> Config {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let provider = rocket::Config::figment().merge(("port", port));
    Config::from(provider)
}
