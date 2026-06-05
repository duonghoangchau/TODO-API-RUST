use std::env;

pub struct EnvConfig;

impl EnvConfig {
    pub fn validate() -> Result<(), env::VarError> {
        for key in ["APP_HOST", "APP_PORT", "DATABASE_URL", "JWT_SECRET"] {
            env::var(key)?;
        }

        Ok(())
    }
}
