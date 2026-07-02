use crate::AppState;
use crate::features::todo::model::CreateTaskToDo;

use axum::Json;
use axum::extract::State;
use serde_json::json;

pub async fn get_todos(State(state): State<AppState>) -> Json<serde_json::Value> {
    let todos = state.todo_service.get_all().await;

    let response = match todos {
        Ok(todos) => json!(todos),
        Err(err) => json!({ "status": "error", "message": err.to_string() }),
    };
    Json(response)
}
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskToDo>,
) -> Json<serde_json::Value> {
    let todo = state.todo_service.create(payload).await;

    let response = match todo {
        Ok(todo) => json!(todo),
        Err(err) => json!({ "status": "error", "message": err.to_string() }),
    };
    Json(response)
}
