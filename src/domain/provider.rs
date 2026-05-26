use async_trait::async_trait;
use std::sync::Arc;

use super::types::*;
use super::user::*;
use super::group::*;
use super::expense::*;
use super::payment::*;
use super::balance::*;
use super::invite::*;
use super::activity::*;

#[async_trait]
pub trait RepositoryProvider:
    UserRepository
    + GroupRepository
    + ExpenseRepository
    + PaymentRepository
    + BalanceRepository
    + InviteRepository
    + ActivityRepository
{}

pub type DynRepos = Arc<dyn RepositoryProvider>;