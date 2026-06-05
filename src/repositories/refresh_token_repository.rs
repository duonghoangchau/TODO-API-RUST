use chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::{error::AppError, models::refresh_token::RefreshToken};

// Dữ liệu tối thiểu để lưu một refresh token mới vào DB.
pub struct CreateRefreshTokenParams {
    pub user_id: i64,
    pub token: String,
    pub expires_at: NaiveDateTime,
}

// Repository này quản lý vòng đời refresh token.
pub struct RefreshTokenRepository;

impl RefreshTokenRepository {
    // Lưu refresh token sau khi register hoặc login thành công.
    pub async fn create(
        pool: &PgPool,
        params: CreateRefreshTokenParams,
    ) -> Result<RefreshToken, AppError> {
        let refresh_token = sqlx::query_as::<_, RefreshToken>(
            r#"
            INSERT INTO refresh_tokens (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token, expires_at, revoked_at, created_at
            "#,
        )
        .bind(params.user_id)
        .bind(params.token)
        .bind(params.expires_at)
        .fetch_one(pool)
        .await?;

        Ok(refresh_token)
    }

    // Chỉ lấy token còn active, chưa bị revoke.
    pub async fn find_active_by_token(
        pool: &PgPool,
        token: &str,
    ) -> Result<Option<RefreshToken>, AppError> {
        let refresh_token = sqlx::query_as::<_, RefreshToken>(
            r#"
            SELECT id, user_id, token, expires_at, revoked_at, created_at
            FROM refresh_tokens
            WHERE token = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(token)
        .fetch_optional(pool)
        .await?;

        Ok(refresh_token)
    }

    // Thu hồi token khi logout để ngăn việc tái sử dụng.
    pub async fn revoke_by_token(pool: &PgPool, token: &str) -> Result<bool, AppError> {
        let rows_affected = sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW()
            WHERE token = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(token)
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }
}
