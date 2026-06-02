use async_graphql::{Object, Context, Result, ID};
use crate::domain::provider::DynRepos;
use crate::domain::payment::Payment;
use crate::domain::user::UserRepository;
use super::user::UserType;

pub struct PaymentType(pub Payment);

impl From<Payment> for PaymentType {
    fn from(p: Payment) -> Self {
        Self(p)
    }
}

#[Object]
impl PaymentType {
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

    async fn paid_to(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let repos = ctx.data::<DynRepos>()?;
        let user = repos.get_user(self.0.paid_to).await?;
        Ok(user.map(UserType::from))
    }

    async fn amount(&self) -> i64 {
        self.0.amount
    }

    async fn currency(&self) -> &str {
        &self.0.currency
    }

    async fn note(&self) -> Option<&str> {
        self.0.note.as_deref()
    }

    async fn created_at(&self) -> i64 {
        self.0.created_at
    }
}
