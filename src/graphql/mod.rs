pub mod types;
pub mod query;
pub mod mutation;
pub mod loaders;

use async_graphql::{Schema, EmptySubscription};

pub type SplitSchema = Schema<query::QueryRoot, mutation::MutationRoot, EmptySubscription>;

pub fn build_schema(repos: crate::domain::provider::DynRepos) -> SplitSchema {
    Schema::build(
        query::QueryRoot,
        mutation::MutationRoot,
        EmptySubscription,
    )
    .data(repos)
    .finish()
}

/// Builds a schema without a RepositoryProvider for compilation/playground.
/// Queries that access repos will return runtime errors.
pub fn build_schema_standalone() -> SplitSchema {
    Schema::build(
        query::QueryRoot,
        mutation::MutationRoot,
        EmptySubscription,
    )
    .finish()
}
