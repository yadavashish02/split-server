use async_trait::async_trait;
use super::types::*;
use crate::domain::category::CategoryId;
use crate::domain::group::GroupId;
use crate::domain::user::UserId;

pub type ExpenseId = uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SplitType {
    Equal,
    Exact,
    Percentage,
    Shares,
}

#[derive(Debug, Clone)]
pub struct Expense {
    pub id: ExpenseId,
    pub group_id: GroupId,
    pub paid_by: UserId,
    pub description: Option<String>,
    pub amount: i64,
    pub currency: CurrencyCode,
    pub split_type: SplitType,
    pub category_id: Option<CategoryId>,
    pub created_at: Timestamp,
    pub updated_at: Option<Timestamp>,
    pub deleted_at: Option<Timestamp>,
}

#[derive(Debug, Clone)]
pub struct ExpenseSplit {
    pub expense_id: ExpenseId,
    pub user_id: UserId,
    pub amount_owed: i64,
}

#[async_trait]
pub trait ExpenseRepository: Send + Sync {
    async fn create_expense(
        &self,
        expense: Expense,
        splits: Vec<ExpenseSplit>,
    ) -> RepoResult<Expense>;

    async fn get_expense(&self, expense_id: ExpenseId) -> RepoResult<Option<Expense>>;

    async fn get_group_expenses(
        &self,
        group_id: GroupId,
        limit: i64,
        created_before: Option<Timestamp>,
    ) -> RepoResult<Vec<Expense>>;

    async fn update_expense(
        &self,
        expense: Expense,
        splits: Vec<ExpenseSplit>,
    ) -> RepoResult<Expense>;

    /// Soft-delete: sets deleted_at timestamp.
    async fn delete_expense(&self, expense_id: ExpenseId) -> RepoResult<()>;
}