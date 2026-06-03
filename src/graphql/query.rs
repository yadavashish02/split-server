use super::types::category::CategoryType;
use super::types::currency::CurrencyType;
use super::types::group::GroupType;
use super::types::user::UserType;
use crate::domain::category::CategoryRepository;
use crate::domain::currency::CurrencyRepository;
use crate::domain::group::GroupRepository;
use crate::domain::provider::DynRepos;
use crate::domain::user::UserRepository;
use async_graphql::{Context, ID, Object, Result};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns the currently authenticated user.
    /// TODO: Extract user_id from auth context once auth is wired.
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        Err("Not implemented: auth context required".into())
    }

    /// Look up a user by ID.
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<Option<UserType>> {
        let repos = ctx.data::<DynRepos>()?;
        let user_id: uuid::Uuid = id.parse()?;
        let user = UserRepository::get_user(repos.as_ref(), user_id).await?;
        Ok(user.map(UserType::from))
    }

    /// Look up a group by ID.
    async fn group(&self, ctx: &Context<'_>, id: ID) -> Result<Option<GroupType>> {
        let repos = ctx.data::<DynRepos>()?;
        let group_id: uuid::Uuid = id.parse()?;
        let group = GroupRepository::get_group(repos.as_ref(), group_id).await?;
        Ok(group.map(GroupType::from))
    }

    /// List all supported currencies.
    async fn currencies(&self, ctx: &Context<'_>) -> Result<Vec<CurrencyType>> {
        let repos = ctx.data::<DynRepos>()?;
        let currencies = CurrencyRepository::get_all(repos.as_ref()).await?;
        Ok(currencies.into_iter().map(CurrencyType::from).collect())
    }

    /// List all categories.
    async fn categories(&self, ctx: &Context<'_>) -> Result<Vec<CategoryType>> {
        let repos = ctx.data::<DynRepos>()?;
        let categories = CategoryRepository::get_all(repos.as_ref()).await?;
        Ok(categories.into_iter().map(CategoryType::from).collect())
    }
}
