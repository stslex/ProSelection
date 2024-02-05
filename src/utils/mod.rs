use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub mod jwt_util;

#[async_trait]
pub trait AppHasher {
    async fn hash(&self) -> String;
}

#[async_trait]
impl AppHasher for str {
    async fn hash(&self) -> String {
        let mut hasher = DefaultHasher::new();
        let value = &self;
        value.hash(&mut hasher);
        hasher.finish().to_string()
    }
}

#[async_trait]
pub trait Mapper<T> {
    async fn map(&self) -> T;
}
