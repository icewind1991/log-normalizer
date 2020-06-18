mod data;
mod database;
mod normalized;
mod raw;

use crate::database::store_log;
use crate::normalized::NormalizedLog;
use main_error::MainError;
use sqlx::{postgres::PgQueryAs, PgPool};

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let database_url = dotenv::var("DATABASE_URL")?;
    let raw_database_url = dotenv::var("RAW_DATABASE_URL")?;

    let pool = PgPool::builder().max_size(2).build(&database_url).await?;
    let raw_pool = PgPool::builder()
        .max_size(2)
        .build(&raw_database_url)
        .await?;

    let max = get_max_log(&raw_pool).await?;
    let from = get_max_stored_log(&pool).await?;

    for id in (from + 1)..=max {
        print!("{} ", id);
        if let Some(log) = get_log(&raw_pool, id).await? {
            println!("{}", log.info.map);
            store_log(&pool, id, &log).await?;
        } else {
            println!("invalid");
        }
    }

    Ok(())
}

async fn get_max_stored_log(pool: &PgPool) -> Result<i32, MainError> {
    Ok(sqlx::query!(r#"SELECT MAX(id) as id from logs"#)
        .fetch_one(pool)
        .await?
        .id
        .unwrap_or_default())
}

async fn get_max_log(pool: &PgPool) -> Result<i32, MainError> {
    let row: (i32,) = sqlx::query_as(r#"SELECT MAX(id) as id from logs_raw"#)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

async fn get_log(pool: &PgPool, id: i32) -> Result<Option<NormalizedLog>, MainError> {
    let row: (serde_json::Value,) =
        sqlx::query_as(r#"SELECT json as id from logs_raw where id = $1"#)
            .bind(id)
            .fetch_one(pool)
            .await?;

    if is_valid(&row.0) {
        Ok(serde_json::from_value(row.0).ok())
    } else {
        Ok(None)
    }
}

fn is_valid(value: &serde_json::Value) -> bool {
    if value.get("success").is_none() {
        return false;
    }
    if value.get("success").unwrap().as_bool().unwrap_or_default() == false {
        return false;
    }

    let rounds = value
        .get("rounds")
        .or_else(|| value.get("info").map(|info| info.get("rounds")).unwrap())
        .unwrap();

    for round in rounds.as_array().unwrap() {
        if round.get("length").unwrap().as_i64().unwrap() < 0 {
            return false;
        }
    }

    true
}
