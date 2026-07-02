use crate::AppState;

use axum::Json;
use axum::extract::State;
use serde_json::json;

pub async fn get_todos(State(state): State<AppState>) -> Json<serde_json::Value> {
    let todos = state.todo_service.get_all().await;

    let response = match todos {
        Ok(todos) => json!({ "status": "success", "data": todos }),
        Err(err) => json!({ "status": "error", "message": err.to_string() }),
    };
    Json(response)
}
pub async fn create_todo() {}
