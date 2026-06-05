use bcrypt::{DEFAULT_COST, hash, verify};

use crate::error::AppError;

// Hash password trước khi lưu DB để không bao giờ lưu plain text.
pub fn hash_password(password: &str) -> Result<String, AppError> {
    Ok(hash(password, DEFAULT_COST)?)
}

// So sánh password người dùng nhập với password hash trong database.
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    Ok(verify(password, password_hash)?)
}
