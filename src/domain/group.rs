use async_trait::async_trait;
use super::types::*;
use crate::domain::user::UserId;

pub type GroupId = uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Group {
    pub id: GroupId,
}

#[async_trait]
pub trait GroupRepository: Send + Sync {
    async fn create_group(&self, group: Group) -> RepoResult<Group>;
    async fn get_group(&self, group_id: GroupId) -> RepoResult<Option<Group>>;
    async fn get_user_groups(&self, user_id: UserId) -> RepoResult<Vec<Group>>;
    async fn add_member(&self, group_id: GroupId, user_id: UserId) -> RepoResult<()>;
}