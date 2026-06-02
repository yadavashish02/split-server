use async_trait::async_trait;
use super::types::*;
use crate::domain::user::UserId;

pub type GroupId = uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Group {
    pub id: GroupId,
    pub name: String,
    pub created_by: UserId,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupRole {
    Admin,
    Member,
}

#[derive(Debug, Clone)]
pub struct GroupMember {
    pub group_id: GroupId,
    pub user_id: UserId,
    pub role: GroupRole,
    pub joined_at: Timestamp,
}

#[async_trait]
pub trait GroupRepository: Send + Sync {
    async fn create_group(&self, group: Group) -> RepoResult<Group>;
    async fn get_group(&self, group_id: GroupId) -> RepoResult<Option<Group>>;
    async fn get_user_groups(&self, user_id: UserId, limit: i64, created_before: Option<Timestamp>) -> RepoResult<Vec<Group>>;
    async fn add_member(&self, group_id: GroupId, user_id: UserId, role: GroupRole) -> RepoResult<()>;
    async fn remove_member(&self, group_id: GroupId, user_id: UserId) -> RepoResult<()>;
    async fn get_members(&self, group_id: GroupId, limit: i64, joined_before: Option<Timestamp>) -> RepoResult<Vec<GroupMember>>;
}