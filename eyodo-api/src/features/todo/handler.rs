use super::model::{NewTodo, TaskQueryParams};
use super::model_response::TodoResponse;

use crate::AppState;
use crate::error::AppError;

use axum::{
    Json,
    extract::{Query, State},
};

pub async fn get_todos(
    State(state): State<AppState>,
    Query(params): Query<TaskQueryParams>,
) -> Result<Json<Vec<TodoResponse>>, AppError> {
    let todos = state.todo_service.get_all(params.filter).await?;

    Ok(Json(todos))
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<NewTodo>,
) -> Result<Json<Vec<TodoResponse>>, AppError> {
    let todos = state.todo_service.create(payload).await?;
    Ok(Json(todos))
}

pub async fn complete_todo(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<Vec<TodoResponse>>, AppError> {
    let id = payload
        .get("id")
        .expect("Pas d'id renseigné")
        .as_i64()
        .unwrap_or(-1) as i32;
    let todos = state.todo_service.complete_todo(id).await?;
    Ok(Json(todos))
}
