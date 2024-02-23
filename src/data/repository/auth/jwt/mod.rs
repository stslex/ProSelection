use self::objects::{JwtGeneratorError, JwtResult};

mod generator;
pub mod objects;

#[async_trait]
pub trait JwtGenerator {
    async fn generate(&self) -> Result<JwtResult, JwtGeneratorError>;
    async fn generate_access(&self) -> Result<String, JwtGeneratorError>;
    async fn generate_refresh(&self) -> Result<String, JwtGeneratorError>;
    async fn generate_token(
        &self,
        secret: &[u8],
        exp_days: i64,
    ) -> Result<String, JwtGeneratorError>;
}
