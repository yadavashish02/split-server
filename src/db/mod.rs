use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, SqlitePool};
use std::str::FromStr;

pub async fn init_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // 1. Configure SQLite specifically for our app
    let options = SqliteConnectOptions::from_str(database_url)?
        .pragma("journal_mode", "WAL") // Required for concurrent performance
        .pragma("foreign_keys", "ON") // Enforce relational integrity
        .pragma("synchronous", "NORMAL") // Best performance/safety balance for WAL
        .pragma("temp_store", "MEMORY") // Keep temp tables (like current_user) in memory
        .pragma("cache_size", "-200000") // 200MB cache
        .disable_statement_logging();

    // 2. Create and return the pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    Ok(pool)
}
