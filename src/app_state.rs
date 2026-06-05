use crate::config::app::AppConfig;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db_pool: PgPool,
}

impl AppState {
    pub fn new(config: AppConfig, db_pool: PgPool) -> Self {
        Self { config, db_pool }
    }
}
