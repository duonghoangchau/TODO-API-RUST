use axum::{Router, routing::post};

use crate::{app_state::AppState, handlers::auth_handler};

// Router này gom toàn bộ endpoint liên quan tới xác thực người dùng.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
        .route("/refresh", post(auth_handler::refresh))
        .route("/logout", post(auth_handler::logout))
}
