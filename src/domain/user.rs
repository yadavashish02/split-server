use async_trait::async_trait;
use super::types::*;

pub type UserId = uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> RepoResult<User>;
    async fn get_user(&self, user_id: UserId) -> RepoResult<Option<User>>;
    async fn get_users(&self, ids: Vec<UserId>) -> RepoResult<Vec<User>>;
}