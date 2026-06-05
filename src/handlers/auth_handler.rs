use axum::{Json, extract::State, http::StatusCode};
use serde_json::Value;

use crate::{
    app_state::AppState,
    dto::auth::{LoginRequest, RefreshTokenRequest, RegisterRequest},
    error::AppError,
    services::auth_service::AuthService,
    utils::response,
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let auth_response = AuthService::register(&state.db_pool, payload).await?;
    Ok((
        StatusCode::CREATED,
        response::success("Register successfully", auth_response),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let auth_response = AuthService::login(&state.db_pool, payload).await?;
    Ok((
        StatusCode::OK,
        response::success("Login successfully", auth_response),
    ))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let refresh_response = AuthService::refresh(&state.db_pool, payload).await?;
    Ok((
        StatusCode::OK,
        response::success("Refresh token successfully", refresh_response),
    ))
}

pub async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    AuthService::logout(&state.db_pool, payload).await?;
    Ok((StatusCode::OK, response::message("Logout successfully")))
}
