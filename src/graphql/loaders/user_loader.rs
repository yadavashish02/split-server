use crate::domain::provider::DynRepos;
use crate::domain::user::{User, UserId, UserRepository};
use async_graphql::dataloader::Loader;
use std::collections::HashMap;
use std::sync::Arc;

pub struct UserLoader {
    repos: DynRepos,
}

impl UserLoader {
    pub fn new(repos: DynRepos) -> Self {
        Self { repos }
    }
}

impl Loader<UserId> for UserLoader {
    type Value = User;
    type Error = Arc<anyhow::Error>;

    fn load(
        &self,
        keys: &[UserId],
    ) -> impl std::future::Future<Output = Result<HashMap<UserId, Self::Value>, Self::Error>> + Send
    {
        let repos = self.repos.clone();
        let keys = keys.to_vec();
        async move {
            let users = repos.get_users(keys).await.map_err(Arc::new)?;
            Ok(users.into_iter().map(|u| (u.id, u)).collect())
        }
    }
}
