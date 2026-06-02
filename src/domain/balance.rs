use async_trait::async_trait;
use super::types::*;
use crate::domain::expense::{Expense, ExpenseSplit};
use crate::domain::group::GroupId;
use crate::domain::user::UserId;

/// Pairwise balance: from_user owes to_user in a specific currency.
#[derive(Debug, Clone)]
pub struct PairwiseBalance {
    pub group_id: GroupId,
    pub from_user: UserId,
    pub to_user: UserId,
    pub currency: CurrencyCode,
    pub amount: i64,
}

/// Net balance for a user in a group (materialized via triggers).
#[derive(Debug, Clone)]
pub struct NetBalance {
    pub group_id: GroupId,
    pub user_id: UserId,
    pub currency: CurrencyCode,
    pub net_amount: i64,
}

#[async_trait]
pub trait BalanceRepository: Send + Sync {
    /// Returns materialized net balances for all members of a group.
    async fn get_group_balances(
        &self,
        group_id: GroupId,
    ) -> RepoResult<Vec<NetBalance>>;

    /// Returns pairwise balances between all members of a group.
    async fn get_pairwise_balances(
        &self,
        group_id: GroupId,
    ) -> RepoResult<Vec<PairwiseBalance>>;

    /// Inserts pairwise ledger entries for an expense.
    /// Net balances are updated automatically via DB triggers.
    async fn update_balances_for_expense(
        &self,
        expense: &Expense,
        splits: &[ExpenseSplit],
    ) -> RepoResult<()>;

    // Note: update_balances_for_payment is NOT needed here.
    // The payment_settle_balance trigger automatically inserts
    // reverse entries into user_balances when a payment is created.
}