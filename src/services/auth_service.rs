use chrono::Utc;
use sqlx::PgPool;

use crate::{
    dto::auth::{
        AuthResponse, LoginRequest, RefreshTokenRequest, RefreshTokenResponse, RegisterRequest,
    },
    error::AppError,
    models::user::User,
    repositories::{
        refresh_token_repository::{CreateRefreshTokenParams, RefreshTokenRepository},
        role_repository::RoleRepository,
        user_repository::{CreateUserParams, UserRepository},
    },
    utils::{jwt, password},
};

// Service này chứa toàn bộ nghiệp vụ auth tách khỏi tầng HTTP.
pub struct AuthService;

impl AuthService {
    // Đăng ký user mới, gán role USER mặc định và phát token ngay sau khi tạo tài khoản.
    pub async fn register(
        pool: &PgPool,
        payload: RegisterRequest,
    ) -> Result<AuthResponse, AppError> {
        validate_register_payload(&payload)?;

        let email = normalize_email(&payload.email);
        if UserRepository::find_by_email(pool, &email).await?.is_some() {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        let user_role = RoleRepository::find_by_name(pool, "USER")
            .await?
            .ok_or_else(|| {
                AppError::Config("Default USER role was not found. Run seed first.".to_string())
            })?;

        let password_hash = password::hash_password(payload.password.trim())?;
        let user = UserRepository::create(
            pool,
            CreateUserParams {
                role_id: user_role.id,
                full_name: payload.full_name.trim().to_string(),
                email,
                password_hash,
            },
        )
        .await
        .map_err(map_create_user_error)?;

        build_auth_response(pool, user).await
    }

    // Đăng nhập bằng cách kiểm tra email, trạng thái user và password đã hash.
    pub async fn login(pool: &PgPool, payload: LoginRequest) -> Result<AuthResponse, AppError> {
        validate_login_payload(&payload)?;

        let email = normalize_email(&payload.email);
        let user = UserRepository::find_by_email(pool, &email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

        if !user.is_active {
            return Err(AppError::Unauthorized(
                "User account is inactive".to_string(),
            ));
        }

        let is_valid_password =
            password::verify_password(payload.password.trim(), &user.password_hash)?;
        if !is_valid_password {
            return Err(AppError::Unauthorized(
                "Invalid email or password".to_string(),
            ));
        }

        build_auth_response(pool, user).await
    }

    // Refresh chỉ tạo access token mới, còn refresh token cũ vẫn được giữ nguyên.
    pub async fn refresh(
        pool: &PgPool,
        payload: RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, AppError> {
        let refresh_token = payload.refresh_token.trim();
        if refresh_token.is_empty() {
            return Err(AppError::Validation(
                "Refresh token is required".to_string(),
            ));
        }

        let stored_token = RefreshTokenRepository::find_active_by_token(pool, refresh_token)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid refresh token".to_string()))?;

        if stored_token.expires_at < Utc::now().naive_utc() {
            return Err(AppError::Unauthorized(
                "Refresh token has expired".to_string(),
            ));
        }

        let user = UserRepository::find_by_id(pool, stored_token.user_id)
            .await?
            .ok_or_else(|| {
                AppError::Unauthorized("User not found for this refresh token".to_string())
            })?;

        if !user.is_active {
            return Err(AppError::Unauthorized(
                "User account is inactive".to_string(),
            ));
        }

        Ok(RefreshTokenResponse {
            access_token: jwt::generate_access_token(user.id)?,
            token_type: "Bearer".to_string(),
        })
    }

    // Logout hiện tại thu hồi refresh token để lần sau không dùng lại được nữa.
    pub async fn logout(pool: &PgPool, payload: RefreshTokenRequest) -> Result<(), AppError> {
        let refresh_token = payload.refresh_token.trim();
        if refresh_token.is_empty() {
            return Err(AppError::Validation(
                "Refresh token is required".to_string(),
            ));
        }

        let revoked = RefreshTokenRepository::revoke_by_token(pool, refresh_token).await?;
        if !revoked {
            return Err(AppError::Unauthorized("Invalid refresh token".to_string()));
        }

        Ok(())
    }
}

// Hàm dùng chung để tạo access token + refresh token sau register/login.
async fn build_auth_response(pool: &PgPool, user: User) -> Result<AuthResponse, AppError> {
    let access_token = jwt::generate_access_token(user.id)?;
    let refresh_token = jwt::generate_refresh_token();
    let expires_at = jwt::refresh_token_expires_at()?;

    RefreshTokenRepository::create(
        pool,
        CreateRefreshTokenParams {
            user_id: user.id,
            token: refresh_token.clone(),
            expires_at,
        },
    )
    .await?;

    Ok(AuthResponse::from_user(user, access_token, refresh_token))
}

// Validate dữ liệu đầu vào ở tầng service để handler gọn và business rule tập trung.
fn validate_register_payload(payload: &RegisterRequest) -> Result<(), AppError> {
    if payload.full_name.trim().is_empty() {
        return Err(AppError::Validation("Full name is required".to_string()));
    }

    if payload.email.trim().is_empty() {
        return Err(AppError::Validation("Email is required".to_string()));
    }

    if !payload.email.contains('@') {
        return Err(AppError::Validation("Email is invalid".to_string()));
    }

    if payload.password.trim().len() < 6 {
        return Err(AppError::Validation(
            "Password must be at least 6 characters".to_string(),
        ));
    }

    Ok(())
}

// Login chỉ cần kiểm tra các trường bắt buộc trước khi query database.
fn validate_login_payload(payload: &LoginRequest) -> Result<(), AppError> {
    if payload.email.trim().is_empty() {
        return Err(AppError::Validation("Email is required".to_string()));
    }

    if payload.password.trim().is_empty() {
        return Err(AppError::Validation("Password is required".to_string()));
    }

    Ok(())
}

// Chuẩn hóa email để tránh duplicate do khác hoa/thường hoặc khoảng trắng.
fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

// Nếu gặp unique constraint ở DB thì đổi sang lỗi nghiệp vụ dễ hiểu hơn.
fn map_create_user_error(error: AppError) -> AppError {
    match error {
        AppError::Database(sqlx::Error::Database(db_error))
            if db_error.code().as_deref() == Some("23505") =>
        {
            AppError::Conflict("Email already exists".to_string())
        }
        other => other,
    }
}
