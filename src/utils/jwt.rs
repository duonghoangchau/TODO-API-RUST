use std::env;

use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

// Claims là payload sẽ được nhúng vào access token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String,
}

// Tạo access token ngắn hạn để client dùng gọi API protected.
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

// Hàm này sẽ hữu ích khi mình làm auth middleware để đọc user_id từ bearer token.
pub fn decode_access_token(token: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret()?.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

// Refresh token hiện được sinh ngẫu nhiên dưới dạng UUID để lưu DB và tra cứu nhanh.
pub fn generate_refresh_token() -> String {
    Uuid::new_v4().to_string()
}

// Tính thời điểm hết hạn cho refresh token dựa trên biến môi trường.
pub fn refresh_token_expires_at() -> Result<NaiveDateTime, AppError> {
    let expiration = Utc::now() + Duration::days(refresh_token_expire_days()?);
    Ok(expiration.naive_utc())
}

// Đọc secret dùng để ký và verify JWT.
fn jwt_secret() -> Result<String, AppError> {
    Ok(env::var("JWT_SECRET")?)
}

// Thời gian sống của access token nên ngắn để giảm rủi ro lộ token.
fn access_token_expire_minutes() -> Result<i64, AppError> {
    let value = env::var("JWT_ACCESS_TOKEN_EXPIRE_MINUTES")
        .unwrap_or_else(|_| "15".to_string())
        .parse()
        .map_err(|_| {
            AppError::Config("JWT_ACCESS_TOKEN_EXPIRE_MINUTES must be a valid integer".to_string())
        })?;

    Ok(value)
}

// Refresh token có thể sống lâu hơn vì đã được lưu và kiểm soát trong database.
fn refresh_token_expire_days() -> Result<i64, AppError> {
    let value = env::var("JWT_REFRESH_TOKEN_EXPIRE_DAYS")
        .unwrap_or_else(|_| "30".to_string())
        .parse()
        .map_err(|_| {
            AppError::Config("JWT_REFRESH_TOKEN_EXPIRE_DAYS must be a valid integer".to_string())
        })?;

    Ok(value)
}
