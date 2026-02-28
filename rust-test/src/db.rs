use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::net::{IpAddr, SocketAddr};
use tokio::net::lookup_host;
use tokio::time::{Duration, sleep};

pub type DbPool = Pool<Postgres>;

fn must_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Missing {key}"))
}

async fn resolve_prefer_ipv4(host: &str, port: u16) -> (String, u16) {
    let addrs: Vec<SocketAddr> = lookup_host((host, port))
        .await
        .unwrap_or_else(|e| panic!("Failed to resolve DB_HOST {host}:{port}: {e}"))
        .collect();

    if let Some(v4) = addrs.iter().find_map(|a| match a.ip() {
        IpAddr::V4(ip) => Some(ip.to_string()),
        IpAddr::V6(_) => None,
    }) {
        return (v4, port);
    }

    (host.to_string(), port)
}

pub async fn get_db_pool() -> DbPool {
    let db_user = must_env("DB_USER");
    let db_password = must_env("DB_PASSWORD");
    let db_host = must_env("DB_HOST");
    let db_port: u16 = must_env("DB_PORT")
        .parse()
        .expect("DB_PORT must be a valid u16 number");
    let db_name = must_env("DB_NAME");

    let (host_for_url, port_for_url) = resolve_prefer_ipv4(&db_host, db_port).await;

    let db_url = format!(
        "postgresql://{}:{}@{}:{}/{}?sslmode=require",
        db_user, db_password, host_for_url, port_for_url, db_name
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
                eprintln!(
                    "DB connect target host={db_host} resolved_host={host_for_url} port={port_for_url}"
                );
                sleep(Duration::from_secs(1)).await;
            }
        }
    }

    panic!("Failed to connect DB after retries");
}
