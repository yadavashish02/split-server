mod db;
mod domain;
mod repository;

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = db::init_pool(&database_url).await?;
    
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(())
}
