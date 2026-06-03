use super::balance::NetBalanceType;
use super::expense::ExpenseType;
use super::invite::InviteType;
use super::user::UserType;
use crate::domain::balance::BalanceRepository;
use crate::domain::expense::ExpenseRepository;
use crate::domain::group::{Group, GroupMember, GroupRepository, GroupRole};
use crate::domain::invite::InviteRepository;
use crate::domain::provider::DynRepos;
use async_graphql::{Context, ID, Object, Result};

// ── GroupType ──────────────────────────────────────────────

pub struct GroupType(pub Group);

impl From<Group> for GroupType {
    fn from(g: Group) -> Self {
        Self(g)
    }
}

#[Object]
impl GroupType {
    async fn id(&self) -> ID {
        ID(self.0.id.to_string())
    }

    async fn name(&self) -> &str {
        &self.0.name
    }

    async fn created_by(&self) -> ID {
        ID(self.0.created_by.to_string())
    }

    async fn created_at(&self) -> i64 {
        self.0.created_at
    }

    async fn members(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 100)] limit: i64,
        before: Option<i64>,
    ) -> Result<Vec<GroupMemberType>> {
        let repos = ctx.data::<DynRepos>()?;
        let members = repos.get_members(self.0.id, limit, before).await?;
        Ok(members.into_iter().map(GroupMemberType::from).collect())
    }

    async fn expenses(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 50)] limit: i64,
        before: Option<i64>,
    ) -> Result<Vec<ExpenseType>> {
        let repos = ctx.data::<DynRepos>()?;
        let expenses = repos.get_group_expenses(self.0.id, limit, before).await?;
        Ok(expenses.into_iter().map(ExpenseType::from).collect())
    }

    async fn balances(&self, ctx: &Context<'_>) -> Result<Vec<NetBalanceType>> {
        let repos = ctx.data::<DynRepos>()?;
        let balances = repos.get_group_balances(self.0.id).await?;
        Ok(balances.into_iter().map(NetBalanceType::from).collect())
    }

    async fn invites(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 50)] limit: i64,
        before: Option<i64>,
    ) -> Result<Vec<InviteType>> {
        let repos = ctx.data::<DynRepos>()?;
        let invites = repos.get_group_invites(self.0.id, limit, before).await?;
        Ok(invites.into_iter().map(InviteType::from).collect())
    }
}

// ── GroupMemberType ────────────────────────────────────────

pub struct GroupMemberType(pub GroupMember);

impl From<GroupMember> for GroupMemberType {
    fn from(m: GroupMember) -> Self {
        Self(m)
    }
}

#[Object]
impl GroupMemberType {
    async fn user_id(&self) -> ID {
        ID(self.0.user_id.to_string())
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let repos = ctx.data::<DynRepos>()?;
        let user = repos.get_user(self.0.user_id).await?;
        Ok(user.map(UserType::from))
    }

    async fn role(&self) -> &str {
        match self.0.role {
            GroupRole::Admin => "admin",
            GroupRole::Member => "member",
        }
    }

    async fn joined_at(&self) -> i64 {
        self.0.joined_at
    }
}
