use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct Db;

impl Db {
    pub async fn config() -> Pool<Postgres> {
        let db_connection_str = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(
                std::env::var("DATABASE_ACQUIRED_TIMEOUT")
                    .unwrap_or(String::from("10"))
                    .parse()
                    .unwrap_or(10),
            ))
            .connect(&db_connection_str)
            .await
            .expect("can connect to database");

        pool
    }
}
