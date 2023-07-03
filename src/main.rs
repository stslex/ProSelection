#![feature(decl_macro)]

mod config;

fn main() {
    rocket::custom(config::from_env());
}
