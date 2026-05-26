use async_trait::async_trait;
use super::types::*;

pub type PaymentId = uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Payment {
    pub id: PaymentId,
}

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn create_payment(&self, payment: Payment) -> RepoResult<Payment>;
}