use serde::Serialize;

use crate::models::user::User;

#[derive(Debug, Serialize)]
pub struct AuthUserResponse {
    pub id: i64,
    pub role_id: i64,
    pub full_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub user: AuthUserResponse,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub token_type: String,
}

impl AuthResponse {
    pub fn from_user(user: User, access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            user: AuthUserResponse::from(user),
        }
    }
}

impl From<User> for AuthUserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            role_id: user.role_id,
            full_name: user.full_name,
            email: user.email,
            avatar_url: user.avatar_url,
            is_active: user.is_active,
        }
    }
}
