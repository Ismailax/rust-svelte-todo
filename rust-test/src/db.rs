use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use tokio::time::{Duration, sleep};

pub type DbPool = Pool<Postgres>;

pub async fn get_db_pool() -> DbPool {
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("DB_USER").unwrap(),
        env::var("DB_PASSWORD").unwrap(),
        env::var("DB_HOST").unwrap(),
        env::var("DB_PORT").unwrap(),
        env::var("DB_NAME").unwrap(),
    );

    for attempt in 1..=10 {
        match PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
        {
            Ok(pool) => return pool,
            Err(e) => {
                eprintln!("DB not ready (attempt {attempt}/10): {e}");
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
    panic!("Failed to connect DB after retries");
}
