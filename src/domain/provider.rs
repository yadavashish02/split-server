use std::sync::Arc;

use super::user::*;
use super::group::*;
use super::expense::*;
use super::payment::*;
use super::balance::*;
use super::invite::*;
use super::activity::*;
use super::category::*;
use super::currency::*;

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
{}

pub type DynRepos = Arc<dyn RepositoryProvider>;