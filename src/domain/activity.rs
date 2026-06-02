use async_trait::async_trait;
use super::types::*;
use crate::domain::group::GroupId;
use crate::domain::user::UserId;

pub type ActivityId = uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ActivityLog {
    pub id: ActivityId,
    pub actor_user_id: UserId,
    pub group_id: Option<GroupId>,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub action: String,
    pub metadata_json: Option<String>,
    pub created_at: Timestamp,
}

#[async_trait]
pub trait ActivityRepository: Send + Sync {
    async fn log_activity(&self, activity: ActivityLog) -> RepoResult<()>;
    async fn get_group_activities(
        &self,
        group_id: GroupId,
        limit: i64,
        created_before: Option<Timestamp>,
    ) -> RepoResult<Vec<ActivityLog>>;
}