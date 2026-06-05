use std::env;

use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String,
}

pub fn generate_access_token(user_id: i64) -> Result<String, AppError> {
    let now = Utc::now();
    let expiration = now + Duration::minutes(access_token_expire_minutes()?);
    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp() as usize,
        iat: now.timestamp() as usize,
        token_type: "access".to_string(),
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret()?.as_bytes()),
    )?)
}

pub fn decode_access_token(token: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret()?.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn generate_refresh_token() -> String {
    Uuid::new_v4().to_string()
}

pub fn refresh_token_expires_at() -> Result<NaiveDateTime, AppError> {
    let expiration = Utc::now() + Duration::days(refresh_token_expire_days()?);
    Ok(expiration.naive_utc())
}

fn jwt_secret() -> Result<String, AppError> {
    Ok(env::var("JWT_SECRET")?)
}

fn access_token_expire_minutes() -> Result<i64, AppError> {
    let value = env::var("JWT_ACCESS_TOKEN_EXPIRE_MINUTES")
        .unwrap_or_else(|_| "15".to_string())
        .parse()
        .map_err(|_| {
            AppError::Config("JWT_ACCESS_TOKEN_EXPIRE_MINUTES must be a valid integer".to_string())
        })?;

    Ok(value)
}

fn refresh_token_expire_days() -> Result<i64, AppError> {
    let value = env::var("JWT_REFRESH_TOKEN_EXPIRE_DAYS")
        .unwrap_or_else(|_| "30".to_string())
        .parse()
        .map_err(|_| {
            AppError::Config("JWT_REFRESH_TOKEN_EXPIRE_DAYS must be a valid integer".to_string())
        })?;

    Ok(value)
}
