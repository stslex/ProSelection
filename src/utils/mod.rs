use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub mod jwt_util;

pub trait AppHasher {
    fn hash(&self) -> String;
}

impl AppHasher for str {
    fn hash(&self) -> String {
        let mut hasher = DefaultHasher::new();
        let value = &self;
        value.hash(&mut hasher);
        hasher.finish().to_string()
    }
}
