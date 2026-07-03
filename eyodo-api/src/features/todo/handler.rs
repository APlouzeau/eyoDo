use crate::AppState;
use crate::features::todo::model::CreateTaskToDo;
use crate::features::todo::model::TaskQueryParams;

use axum::Json;
use axum::extract::Query;
use axum::extract::State;
use dbg;
use serde_json::json;

pub async fn get_todos(
    State(state): State<AppState>,
    Query(params): Query<TaskQueryParams>,
) -> Json<serde_json::Value> {
    let todos = state.todo_service.get_all(params.filter).await;

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
    dbg!(&payload);
    let todo = state.todo_service.create(payload).await;
    let response = match todo {
        Ok(todo) => json!(todo),
        Err(err) => json!({ "status": "error", "message": err.to_string() }),
    };
    Json(response)
}

pub async fn complete_todo(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let id = payload
        .get("id")
        .expect("Pas d'id renseigné")
        .as_i64()
        .unwrap_or(-1) as i32;
    let todo = state.todo_service.complete_todo(id).await;

    let response = match todo {
        Ok(todo) => json!(todo),
        Err(err) => json!({ "status": "error", "message": err.to_string() }),
    };
    Json(response)
}
