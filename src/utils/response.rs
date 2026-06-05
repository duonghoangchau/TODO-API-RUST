use axum::Json;
use serde::Serialize;
use serde_json::{Value, json};

// Helper cho các API trả về dữ liệu kèm message theo format thống nhất.
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

// Helper cho các API chỉ cần trả thông báo thành công, không kèm data.
pub fn message(message: &str) -> Json<Value> {
    Json(json!({
        "success": true,
        "message": message,
    }))
}
