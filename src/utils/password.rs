use bcrypt::{DEFAULT_COST, hash, verify};

use crate::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    Ok(hash(password, DEFAULT_COST)?)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    Ok(verify(password, password_hash)?)
}
