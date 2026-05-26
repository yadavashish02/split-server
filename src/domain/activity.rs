use async_trait::async_trait;
use super::types::*;

pub type ActivityId = uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ActivityLog {
    pub id: ActivityId,
}

#[async_trait]
pub trait ActivityRepository: Send + Sync {
    async fn log_activity(&self, activity: ActivityLog) -> RepoResult<()>;
}