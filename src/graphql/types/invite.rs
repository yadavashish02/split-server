use async_graphql::{Object, Context, Result, ID};
use crate::domain::provider::DynRepos;
use crate::domain::invite::{Invite, InviteStatus};
use crate::domain::user::UserRepository;
use super::user::UserType;

pub struct InviteType(pub Invite);

impl From<Invite> for InviteType {
    fn from(i: Invite) -> Self {
        Self(i)
    }
}

#[Object]
impl InviteType {
    async fn id(&self) -> ID {
        ID(self.0.id.to_string())
    }

    async fn group_id(&self) -> ID {
        ID(self.0.group_id.to_string())
    }

    async fn invited_by(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let repos = ctx.data::<DynRepos>()?;
        let user = repos.get_user(self.0.invited_by).await?;
        Ok(user.map(UserType::from))
    }

    async fn email(&self) -> Option<&str> {
        self.0.email.as_deref()
    }

    async fn invited_user(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let Some(user_id) = self.0.invited_user_id else {
            return Ok(None);
        };
        let repos = ctx.data::<DynRepos>()?;
        let user = repos.get_user(user_id).await?;
        Ok(user.map(UserType::from))
    }

    async fn status(&self) -> &str {
        match self.0.status {
            InviteStatus::Pending => "pending",
            InviteStatus::Accepted => "accepted",
            InviteStatus::Declined => "declined",
            InviteStatus::Expired => "expired",
        }
    }

    async fn token(&self) -> &str {
        &self.0.token
    }

    async fn expires_at(&self) -> Option<i64> {
        self.0.expires_at
    }

    async fn created_at(&self) -> i64 {
        self.0.created_at
    }
}
