use async_graphql::{Object, Context, Result, ID};
use crate::domain::provider::DynRepos;
use crate::domain::expense::{Expense, ExpenseSplit, SplitType};
use crate::domain::user::UserRepository;
use super::user::UserType;

// ── ExpenseType ────────────────────────────────────────────

pub struct ExpenseType(pub Expense);

impl From<Expense> for ExpenseType {
    fn from(e: Expense) -> Self {
        Self(e)
    }
}

#[Object]
impl ExpenseType {
    async fn id(&self) -> ID {
        ID(self.0.id.to_string())
    }

    async fn group_id(&self) -> ID {
        ID(self.0.group_id.to_string())
    }

    async fn paid_by(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let repos = ctx.data::<DynRepos>()?;
        let user = repos.get_user(self.0.paid_by).await?;
        Ok(user.map(UserType::from))
    }

    async fn description(&self) -> Option<&str> {
        self.0.description.as_deref()
    }

    async fn amount(&self) -> i64 {
        self.0.amount
    }

    async fn currency(&self) -> &str {
        &self.0.currency
    }

    async fn split_type(&self) -> &str {
        match self.0.split_type {
            SplitType::Equal => "equal",
            SplitType::Exact => "exact",
            SplitType::Percentage => "percentage",
            SplitType::Shares => "shares",
        }
    }

    async fn category_id(&self) -> Option<&str> {
        self.0.category_id.as_deref()
    }

    async fn created_at(&self) -> i64 {
        self.0.created_at
    }

    async fn updated_at(&self) -> Option<i64> {
        self.0.updated_at
    }
}

// ── ExpenseSplitType ───────────────────────────────────────

pub struct ExpenseSplitType(pub ExpenseSplit);

impl From<ExpenseSplit> for ExpenseSplitType {
    fn from(s: ExpenseSplit) -> Self {
        Self(s)
    }
}

#[Object]
impl ExpenseSplitType {
    async fn user_id(&self) -> ID {
        ID(self.0.user_id.to_string())
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let repos = ctx.data::<DynRepos>()?;
        let user = repos.get_user(self.0.user_id).await?;
        Ok(user.map(UserType::from))
    }

    async fn amount_owed(&self) -> i64 {
        self.0.amount_owed
    }
}
