use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::config::database::DatabaseConfig;

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.acquire_timeout_seconds))
        .connect(&config.url)
        .await
}
