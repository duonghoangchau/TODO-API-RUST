pub mod auth_routes;

use axum::{Json, Router, extract::State, routing::get};
use serde_json::{Value, json};
use tower_http::trace::TraceLayer;

use crate::app_state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/health", get(api_health_check))
        .route("/api/health/db", get(database_health_check))
        .nest("/api/auth", auth_routes::router())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}

async fn api_health_check(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "success": true,
        "message": format!("{} is running", state.config.name),
    }))
}

async fn database_health_check(State(state): State<AppState>) -> Json<Value> {
    let is_connected = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db_pool)
        .await
        .map(|value| value == 1)
        .unwrap_or(false);

    if is_connected {
        Json(json!({
            "success": true,
            "message": "Database connected",
        }))
    } else {
        Json(json!({
            "success": false,
            "message": "Database connection failed",
        }))
    }
}
