use async_trait::async_trait;
use super::types::*;

pub type CategoryId = String;

#[derive(Debug, Clone)]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
    pub icon: Option<String>,
}

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn get_all(&self) -> RepoResult<Vec<Category>>;
    async fn get_by_id(&self, id: &str) -> RepoResult<Option<Category>>;
}
