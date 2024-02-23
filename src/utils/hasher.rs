use super::AppHasher;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[async_trait]
impl AppHasher for str {
    async fn hash(&self) -> String {
        let mut hasher = DefaultHasher::new();
        let value = &self;
        value.hash(&mut hasher);
        hasher.finish().to_string()
    }
}
