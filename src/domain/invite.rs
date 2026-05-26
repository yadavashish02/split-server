use async_trait::async_trait;
use super::types::*;

pub type InviteId = uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Invite {
    pub id: InviteId,
}

#[async_trait]
pub trait InviteRepository: Send + Sync {
    async fn create_invite(&self, invite: Invite) -> RepoResult<Invite>;
    async fn accept_invite(&self, token: String) -> RepoResult<()>;
}