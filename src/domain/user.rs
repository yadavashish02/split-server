use super::types::*;
use async_trait::async_trait;

pub type UserId = uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub created_at: Timestamp,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> RepoResult<User>;
    async fn get_user(&self, user_id: UserId) -> RepoResult<Option<User>>;
    async fn get_users(&self, ids: Vec<UserId>) -> RepoResult<Vec<User>>;
}
