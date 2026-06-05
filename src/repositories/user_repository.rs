use sqlx::PgPool;

use crate::{error::AppError, models::user::User};

// Dữ liệu cần thiết để tạo user mới từ tầng service.
pub struct CreateUserParams {
    pub role_id: i64,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
}

// Repository chỉ tập trung vào truy vấn bảng users.
pub struct UserRepository;

impl UserRepository {
    // Tìm user theo email để phục vụ login hoặc kiểm tra trùng khi register.
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, role_id, full_name, email, password_hash, avatar_url, is_active, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    // Tìm user theo id để phục vụ các flow cần xác nhận chủ thể từ token.
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, role_id, full_name, email, password_hash, avatar_url, is_active, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    // Tạo user mới và trả lại bản ghi vừa insert từ database.
    pub async fn create(pool: &PgPool, params: CreateUserParams) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (role_id, full_name, email, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id, role_id, full_name, email, password_hash, avatar_url, is_active, created_at, updated_at
            "#,
        )
        .bind(params.role_id)
        .bind(params.full_name)
        .bind(params.email)
        .bind(params.password_hash)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}
