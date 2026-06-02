use async_graphql::{Object, Context, Result, InputObject, ID};
use crate::domain::provider::DynRepos;
use crate::domain::user::{User, UserRepository};
use crate::domain::group::{Group, GroupRole, GroupRepository};
use crate::domain::expense::{Expense, ExpenseSplit, SplitType, ExpenseRepository};
use crate::domain::payment::{Payment, PaymentRepository};
use crate::domain::invite::{Invite, InviteStatus, InviteRepository};
use crate::domain::balance::BalanceRepository;
use super::types::user::UserType;
use super::types::group::GroupType;
use super::types::expense::ExpenseType;
use super::types::payment::PaymentType;
use super::types::invite::InviteType;

pub struct MutationRoot;

// ── Input types ────────────────────────────────────────────

#[derive(InputObject)]
pub struct CreateUserInput {
    pub username: String,
}

#[derive(InputObject)]
pub struct CreateGroupInput {
    pub name: String,
}

#[derive(InputObject)]
pub struct SplitInput {
    pub user_id: ID,
    pub amount_owed: i64,
}

#[derive(InputObject)]
pub struct AddExpenseInput {
    pub group_id: ID,
    pub description: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub split_type: String,
    pub category_id: Option<String>,
    pub splits: Vec<SplitInput>,
}

#[derive(InputObject)]
pub struct SettlePaymentInput {
    pub group_id: ID,
    pub paid_to: ID,
    pub amount: i64,
    pub currency: String,
    pub note: Option<String>,
}

#[derive(InputObject)]
pub struct EditExpenseInput {
    pub id: ID,
    pub description: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub split_type: String,
    pub category_id: Option<String>,
    pub splits: Vec<SplitInput>,
}

#[derive(InputObject)]
pub struct InviteMemberInput {
    pub group_id: ID,
    pub email: Option<String>,
    pub invited_user_id: Option<ID>,
}

// ── Mutations ──────────────────────────────────────────────

#[Object]
impl MutationRoot {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<UserType> {
        let repos = ctx.data::<DynRepos>()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;
        let user = User {
            id: uuid::Uuid::now_v7(),
            username: input.username,
            created_at: now,
        };
        let created = repos.create_user(user).await?;
        Ok(UserType::from(created))
    }

    async fn create_group(
        &self,
        ctx: &Context<'_>,
        input: CreateGroupInput,
    ) -> Result<GroupType> {
        let repos = ctx.data::<DynRepos>()?;
        // TODO: Extract user_id from auth context.
        // For now, this will need a `created_by` field in the input or auth middleware.
        return Err("Not implemented: auth context required for created_by".into());
    }

    async fn add_expense(
        &self,
        ctx: &Context<'_>,
        input: AddExpenseInput,
    ) -> Result<ExpenseType> {
        let repos = ctx.data::<DynRepos>()?;

        let group_id: uuid::Uuid = input.group_id.parse()?;
        // TODO: Extract paid_by from auth context.
        return Err("Not implemented: auth context required for paid_by".into());
    }

    async fn settle_payment(
        &self,
        ctx: &Context<'_>,
        input: SettlePaymentInput,
    ) -> Result<PaymentType> {
        let repos = ctx.data::<DynRepos>()?;

        let group_id: uuid::Uuid = input.group_id.parse()?;
        let paid_to: uuid::Uuid = input.paid_to.parse()?;
        // TODO: Extract paid_by from auth context.
        return Err("Not implemented: auth context required for paid_by".into());
    }

    async fn invite_member(
        &self,
        ctx: &Context<'_>,
        input: InviteMemberInput,
    ) -> Result<InviteType> {
        let repos = ctx.data::<DynRepos>()?;

        let group_id: uuid::Uuid = input.group_id.parse()?;
        // TODO: Extract invited_by from auth context.
        return Err("Not implemented: auth context required for invited_by".into());
    }

    async fn accept_invite(
        &self,
        ctx: &Context<'_>,
        token: String,
    ) -> Result<bool> {
        let repos = ctx.data::<DynRepos>()?;
        repos.accept_invite(&token).await?;
        Ok(true)
    }

    async fn edit_expense(
        &self,
        ctx: &Context<'_>,
        input: EditExpenseInput,
    ) -> Result<ExpenseType> {
        let repos = ctx.data::<DynRepos>()?;
        let expense_id: uuid::Uuid = input.id.parse()?;

        // Fetch existing expense to get immutable fields (group_id, paid_by, created_at)
        let existing = ExpenseRepository::get_expense(repos.as_ref(), expense_id)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Expense not found"))?;

        let split_type = match input.split_type.as_str() {
            "equal" => SplitType::Equal,
            "exact" => SplitType::Exact,
            "percentage" => SplitType::Percentage,
            "shares" => SplitType::Shares,
            other => return Err(async_graphql::Error::new(format!("Invalid split_type: {other}"))),
        };

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        let expense = Expense {
            id: expense_id,
            group_id: existing.group_id,
            paid_by: existing.paid_by,
            description: input.description,
            amount: input.amount,
            currency: input.currency,
            split_type,
            category_id: input.category_id,
            created_at: existing.created_at,
            updated_at: Some(now),
            deleted_at: None,
        };

        let splits: Vec<ExpenseSplit> = input.splits.into_iter().map(|s| {
            let user_id: uuid::Uuid = s.user_id.to_string().parse().unwrap();
            ExpenseSplit {
                expense_id,
                user_id,
                amount_owed: s.amount_owed,
            }
        }).collect();

        let updated = ExpenseRepository::update_expense(repos.as_ref(), expense, splits).await?;
        Ok(ExpenseType::from(updated))
    }

    async fn delete_expense(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<bool> {
        let repos = ctx.data::<DynRepos>()?;
        let expense_id: uuid::Uuid = id.parse()?;
        ExpenseRepository::delete_expense(repos.as_ref(), expense_id).await?;
        Ok(true)
    }
}
