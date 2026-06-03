use super::user::UserType;
use crate::domain::balance::{NetBalance, PairwiseBalance};
use crate::domain::provider::DynRepos;
use crate::domain::user::UserRepository;
use async_graphql::{Context, ID, Object, Result};

// ── NetBalanceType ─────────────────────────────────────────

pub struct NetBalanceType(pub NetBalance);

impl From<NetBalance> for NetBalanceType {
    fn from(b: NetBalance) -> Self {
        Self(b)
    }
}

#[Object]
impl NetBalanceType {
    async fn user_id(&self) -> ID {
        ID(self.0.user_id.to_string())
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let repos = ctx.data::<DynRepos>()?;
        let user = repos.get_user(self.0.user_id).await?;
        Ok(user.map(UserType::from))
    }

    async fn currency(&self) -> &str {
        &self.0.currency
    }

    async fn net_amount(&self) -> i64 {
        self.0.net_amount
    }
}

// ── PairwiseBalanceType ────────────────────────────────────

pub struct PairwiseBalanceType(pub PairwiseBalance);

impl From<PairwiseBalance> for PairwiseBalanceType {
    fn from(b: PairwiseBalance) -> Self {
        Self(b)
    }
}

#[Object]
impl PairwiseBalanceType {
    async fn from_user(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let repos = ctx.data::<DynRepos>()?;
        let user = repos.get_user(self.0.from_user).await?;
        Ok(user.map(UserType::from))
    }

    async fn to_user(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let repos = ctx.data::<DynRepos>()?;
        let user = repos.get_user(self.0.to_user).await?;
        Ok(user.map(UserType::from))
    }

    async fn currency(&self) -> &str {
        &self.0.currency
    }

    async fn amount(&self) -> i64 {
        self.0.amount
    }
}
