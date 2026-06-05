use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    #[serde(alias = "name")]
    pub full_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}
