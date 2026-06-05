use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub name: String,
    pub environment: String,
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        let name = env::var("APP_NAME").unwrap_or_else(|_| "Todo API".to_string());
        let environment = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());
        let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .unwrap_or(8000);

        Ok(Self {
            name,
            environment,
            host,
            port,
        })
    }
}
