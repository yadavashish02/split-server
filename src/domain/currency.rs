use super::types::*;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: String,
    pub minor_unit: i32,
}

#[async_trait]
pub trait CurrencyRepository: Send + Sync {
    async fn get_all(&self) -> RepoResult<Vec<Currency>>;
    async fn get_by_code(&self, code: &str) -> RepoResult<Option<Currency>>;
}
