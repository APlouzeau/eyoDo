use crate::AppState;

use axum::Json;
use axum::extract::State;
use dbg;
use serde_json::json;

pub async fn get_users(State(state): State<AppState>) -> Json<serde_json::Value> {
    let users = state.user_service.get_all().await;

    let response = match users {
        Ok(users) => json!(users),
        Err(err) => json!({ "status": "error", "message": err.to_string() }),
    };
    Json(response)
}
