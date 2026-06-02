use async_trait::async_trait;
use super::types::*;
use crate::domain::group::GroupId;
use crate::domain::user::UserId;

pub type InviteId = uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InviteStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
}

#[derive(Debug, Clone)]
pub struct Invite {
    pub id: InviteId,
    pub group_id: GroupId,
    pub invited_by: UserId,
    pub email: Option<String>,
    pub invited_user_id: Option<UserId>,
    pub status: InviteStatus,
    pub token: String,
    pub expires_at: Option<Timestamp>,
    pub created_at: Timestamp,
}

#[async_trait]
pub trait InviteRepository: Send + Sync {
    async fn create_invite(&self, invite: Invite) -> RepoResult<Invite>;
    async fn get_invite_by_token(&self, token: &str) -> RepoResult<Option<Invite>>;
    async fn accept_invite(&self, token: &str) -> RepoResult<()>;
    async fn get_group_invites(&self, group_id: GroupId, limit: i64, created_before: Option<Timestamp>) -> RepoResult<Vec<Invite>>;
}