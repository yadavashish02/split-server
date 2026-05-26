use async_trait::async_trait;
use super::types::*;
use crate::domain::group::GroupId;
use crate::domain::user::UserId;

pub type ExpenseId = uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Expense {
    pub id: ExpenseId,
    pub group_id: GroupId,
}

#[derive(Debug, Clone)]
pub struct ExpenseSplit {
    pub user_id: UserId,
    pub amount: i64,
}

#[async_trait]
pub trait ExpenseRepository: Send + Sync {
    async fn create_expense(
        &self,
        expense: Expense,
        splits: Vec<ExpenseSplit>,
    ) -> RepoResult<Expense>;

    async fn get_group_expenses(&self, group_id: GroupId)
        -> RepoResult<Vec<Expense>>;
}