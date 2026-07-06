use crate::AppState;
use crate::error::AppError;
use crate::features::todo::model::NewToDo;
use crate::features::todo::model::TaskQueryParams;
use crate::features::todo::model::TodoResponse;

use axum::Json;
use axum::extract::Query;
use axum::extract::State;
use dbg;
use serde_json::json;

pub async fn get_todos(
    State(state): State<AppState>,
    Query(params): Query<TaskQueryParams>,
) -> Result<Json<Vec<TodoResponse>>, AppError> {
    let todos = state.todo_service.get_all(params.filter).await?;

    Ok(Json(todos))
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<NewToDo>,
) -> Result<Json<TodoResponse>, AppError> {
    let todo = state.todo_service.create(payload).await?;
    Ok(Json(todo))
}

pub async fn complete_todo(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<TodoResponse>, AppError> {
    let id = payload
        .get("id")
        .expect("Pas d'id renseigné")
        .as_i64()
        .unwrap_or(-1) as i32;
    let todo = state.todo_service.complete_todo(id).await?;
    Ok(Json(todo))
}
