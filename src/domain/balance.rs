use async_trait::async_trait;
use super::types::*;
use crate::domain::group::GroupId;
use crate::domain::expense::{Expense, ExpenseSplit};
use crate::domain::payment::Payment;
use crate::domain::user::UserId;

#[derive(Debug, Clone)]
pub struct UserBalance {
    pub user_id: UserId,
    pub balance: i64,
}

#[async_trait]
pub trait BalanceRepository: Send + Sync {
    async fn get_group_balances(
        &self,
        group_id: GroupId,
    ) -> RepoResult<Vec<UserBalance>>;

    async fn update_balances_for_expense(
        &self,
        expense: &Expense,
        splits: &[ExpenseSplit],
    ) -> RepoResult<()>;

    async fn update_balances_for_payment(
        &self,
        payment: &Payment,
    ) -> RepoResult<()>;
}