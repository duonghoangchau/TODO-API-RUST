use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

// Enum lỗi dùng chung để toàn bộ app trả JSON lỗi theo cùng một format.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Config(String),
    #[error("{0}")]
    Internal(String),
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Hash(#[from] bcrypt::BcryptError),
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Env(#[from] std::env::VarError),
}

impl AppError {
    // Map lỗi nghiệp vụ sang HTTP status code tương ứng.
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Config(_)
            | Self::Internal(_)
            | Self::Database(_)
            | Self::Hash(_)
            | Self::Jwt(_)
            | Self::Env(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Mọi lỗi đều được chuẩn hóa thành JSON để frontend/Postman dễ đọc.
        let status = self.status_code();
        let body = Json(json!({
            "success": false,
            "message": self.to_string(),
        }));

        (status, body).into_response()
    }
}
