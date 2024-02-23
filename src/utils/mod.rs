mod hasher;

#[async_trait]
pub trait AppHasher {
    async fn hash(&self) -> String;
}

#[async_trait]
pub trait Mapper<T> {
    async fn map(&self) -> T;
}
