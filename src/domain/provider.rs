use std::sync::Arc;

use super::activity::*;
use super::balance::*;
use super::category::*;
use super::currency::*;
use super::expense::*;
use super::group::*;
use super::invite::*;
use super::payment::*;
use super::user::*;

pub trait RepositoryProvider:
    UserRepository
    + GroupRepository
    + ExpenseRepository
    + PaymentRepository
    + BalanceRepository
    + InviteRepository
    + ActivityRepository
    + CategoryRepository
    + CurrencyRepository
{
}

pub type DynRepos = Arc<dyn RepositoryProvider>;
