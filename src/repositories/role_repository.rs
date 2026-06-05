use sqlx::PgPool;

use crate::{error::AppError, models::role::Role};

// Repository đơn giản để tra cứu role mặc định cho user mới.
pub struct RoleRepository;

impl RoleRepository {
    pub async fn find_by_name(pool: &PgPool, name: &str) -> Result<Option<Role>, AppError> {
        let role = sqlx::query_as::<_, Role>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM roles
            WHERE name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;

        Ok(role)
    }
}
