mod data;
mod database;
mod normalized;
mod raw;

use crate::database::store_log;
use crate::normalized::NormalizedLog;
use main_error::MainError;
use sqlx::PgPool;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let database_url = dotenv::var("DATABASE_URL")?;

    let content = fs::read_to_string("tests/data/2522305.json").unwrap();
    let parsed: NormalizedLog = serde_json::from_str(&content).unwrap();

    let pool = PgPool::builder().max_size(2).build(&database_url).await?;
    dbg!(store_log(&pool, 2522305, &parsed).await)?;

    Ok(())
}
