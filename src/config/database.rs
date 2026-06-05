use std::env;

#[derive(Clone, Debug)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_seconds: u64,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        let url = env::var("DATABASE_URL")?;
        let max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(10);
        let min_connections = env::var("DATABASE_MIN_CONNECTIONS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(1);
        let acquire_timeout_seconds = env::var("DATABASE_ACQUIRE_TIMEOUT_SECONDS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(10);

        Ok(Self {
            url,
            max_connections,
            min_connections,
            acquire_timeout_seconds,
        })
    }
}
