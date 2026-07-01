use axum::Json;
use serde_json::json;

pub async fn get_todos() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Hello, world!"
    }))
}
pub async fn create_todo() {}
