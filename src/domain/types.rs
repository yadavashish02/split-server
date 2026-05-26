use async_trait::async_trait;
use crate::domain::*;

pub type RepoResult<T> = Result<T, anyhow::Error>;