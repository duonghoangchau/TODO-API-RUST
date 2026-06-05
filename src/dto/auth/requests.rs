use serde::Deserialize;

// Body cho API register. Giữ alias "name" để tương thích với docs cũ nếu cần.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    #[serde(alias = "name")]
    pub full_name: String,
    pub email: String,
    pub password: String,
}

// Body cho API login.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Body dùng chung cho refresh và logout vì cả hai đều cần refresh token.
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}
