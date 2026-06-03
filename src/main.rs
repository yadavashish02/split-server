mod db;
mod domain;
mod graphql;
mod repository;

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Router, extract::State, routing::get};
use dotenvy::dotenv;
use std::env;

async fn graphql_handler(
    State(schema): State<graphql::SplitSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = db::init_pool(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    // Build schema — DynRepos will be injected once a concrete RepositoryProvider exists.
    // For now, the schema compiles but mutations/queries that need repos will error at runtime.
    // TODO: Wire SqlRepositoryProvider(pool) → Arc<dyn RepositoryProvider> → build_schema(repos)

    let schema = graphql::build_schema_standalone();

    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .with_state(schema);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("🚀 GraphQL playground: http://localhost:8080/graphql");
    axum::serve(listener, app).await?;

    Ok(())
}
