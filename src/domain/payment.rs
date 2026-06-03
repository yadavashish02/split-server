use super::types::*;
use crate::domain::group::GroupId;
use crate::domain::user::UserId;
use async_trait::async_trait;

pub type PaymentId = uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Payment {
    pub id: PaymentId,
    pub group_id: GroupId,
    pub paid_by: UserId,
    pub paid_to: UserId,
    pub amount: i64,
    pub currency: CurrencyCode,
    pub note: Option<String>,
    pub created_at: Timestamp,
}

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    /// Inserts payment. The payment_settle_balance trigger
    /// automatically updates user_balances and net_balances.
    async fn create_payment(&self, payment: Payment) -> RepoResult<Payment>;
    async fn get_group_payments(
        &self,
        group_id: GroupId,
        limit: i64,
        created_before: Option<Timestamp>,
    ) -> RepoResult<Vec<Payment>>;
}
