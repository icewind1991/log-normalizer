mod data;
mod database;
mod normalized;
pub mod raw;

use crate::database::{store_log, upgrade};
use crate::normalized::NormalizedLog;
use anyhow::{Context, Error};
use main_error::MainError;
use sqlx::pool::PoolOptions;
use sqlx::PgPool;
use tokio::time::{sleep, Duration};
use tracing::{error, info, instrument};

const OLD_VERSION: i16 = 1;
const VERSION: i16 = 2;

#[tokio::main]
async fn main() -> Result<(), MainError> {
    tracing_subscriber::fmt::init();
    let database_url = dotenvy::var("DATABASE_URL")?;
    let raw_database_url = dotenvy::var("RAW_DATABASE_URL")?;

    loop {
        normalize(&database_url, &raw_database_url).await?;
        sleep(Duration::from_secs(15 * 60)).await;
    }
}

async fn normalize(database_url: &str, raw_database_url: &str) -> Result<(), Error> {
    let pool = PoolOptions::new()
        .max_connections(2)
        .connect(database_url)
        .await
        .context("Failed to connect to log database")?;
    let raw_pool = PoolOptions::new()
        .max_connections(2)
        .connect(raw_database_url)
        .await
        .context("Failed to connect to raw log database")?;

    let max = get_max_log(&raw_pool)
        .await
        .context("Failed to get max raw log")?;
    let old = get_min_old_stored_log(&pool, VERSION)
        .await
        .context("Failed to get min processed old log")?;
    let from = get_max_stored_log(&pool)
        .await
        .context("Failed to get min processed log")?;

    if let Some(old) = old {
        for id in old..=from {
            info!(id = id, from = OLD_VERSION, to = VERSION, "migrating");
            if let Some(log) = get_log(&raw_pool, id).await? {
                upgrade(&pool, id, &log, OLD_VERSION, VERSION).await?;
            } else {
                error!(id = id, "invalid");
            }
        }
    }

    for id in (from + 1)..=max {
        if let Some(log) = get_log(&raw_pool, id).await? {
            info!(id = id, map = display(&log.info.map), "normalizing");
            store_log(&pool, id, &log).await?;
        } else {
            error!(id = id, "invalid");
        }
    }

    Ok(())
}

async fn get_min_old_stored_log(pool: &PgPool, version: i16) -> Result<Option<i32>, Error> {
    Ok(sqlx::query!(
        r#"SELECT MIN(id) as "id" from logs WHERE version < $1"#,
        version
    )
    .fetch_optional(pool)
    .await?
    .and_then(|row| row.id))
}

async fn get_max_stored_log(pool: &PgPool) -> Result<i32, Error> {
    Ok(sqlx::query!(r#"SELECT MAX(id) as id from logs"#)
        .fetch_one(pool)
        .await?
        .id
        .unwrap_or_default())
}

async fn get_max_log(pool: &PgPool) -> Result<i32, Error> {
    let row: (i32,) = sqlx::query_as(r#"SELECT MAX(id) as id from logs_raw"#)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

#[instrument(skip(pool))]
async fn get_log(pool: &PgPool, id: i32) -> Result<Option<NormalizedLog>, Error> {
    let row: (serde_json::Value,) =
        sqlx::query_as(r#"SELECT json as id from logs_raw where id = $1"#)
            .bind(id)
            .fetch_one(pool)
            .await
            .context("failed to get raw log")?;

    if is_valid(&row.0) {
        match serde_json::from_value(row.0) {
            Ok(log) => Ok(Some(log)),
            Err(err) => {
                let formatted_err = format!("{}", err);
                eprintln!("{}", formatted_err);
                if formatted_err.starts_with("Invalid SteamID") {
                    return Ok(None);
                }
                if formatted_err.starts_with("Malformed SteamID") {
                    return Ok(None);
                }
                if formatted_err.starts_with("invalid value: integer ") {
                    return Ok(None);
                }
                if formatted_err.starts_with("invalid type: floating point") {
                    return Ok(None);
                }
                Err(err).context("failed parse raw log")
            }
        }
    } else {
        Ok(None)
    }
}

fn is_valid(value: &serde_json::Value) -> bool {
    if value.get("success").is_none() {
        info!("missing 'success'");
        return false;
    }
    if !value.get("success").unwrap().as_bool().unwrap_or_default() {
        info!("'success' is false");
        return false;
    }

    let rounds = value
        .get("rounds")
        .or_else(|| value.get("info").map(|info| info.get("rounds")).unwrap())
        .unwrap();

    for (index, round) in rounds.as_array().unwrap().iter().enumerate() {
        if round.get("length").unwrap().as_i64().unwrap() < 0 {
            info!(round = index, "round has invalid length");
            return false;
        }
    }

    true
}
