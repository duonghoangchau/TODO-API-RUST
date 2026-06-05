use axum::Json;
use serde::Serialize;
use serde_json::{Value, json};

pub fn success<T>(message: &str, data: T) -> Json<Value>
where
    T: Serialize,
{
    Json(json!({
        "success": true,
        "message": message,
        "data": data,
    }))
}

pub fn message(message: &str) -> Json<Value> {
    Json(json!({
        "success": true,
        "message": message,
    }))
}
