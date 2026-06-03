use super::group::GroupType;
use crate::domain::group::GroupRepository;
use crate::domain::provider::DynRepos;
use crate::domain::user::User;
use async_graphql::{Context, ID, Object, Result};

pub struct UserType(pub User);

impl From<User> for UserType {
    fn from(u: User) -> Self {
        Self(u)
    }
}

#[Object]
impl UserType {
    async fn id(&self) -> ID {
        ID(self.0.id.to_string())
    }

    async fn username(&self) -> &str {
        &self.0.username
    }

    async fn created_at(&self) -> i64 {
        self.0.created_at
    }

    async fn groups(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 50)] limit: i64,
        before: Option<i64>,
    ) -> Result<Vec<GroupType>> {
        let repos = ctx.data::<DynRepos>()?;
        let groups = repos.get_user_groups(self.0.id, limit, before).await?;
        Ok(groups.into_iter().map(GroupType::from).collect())
    }
}
