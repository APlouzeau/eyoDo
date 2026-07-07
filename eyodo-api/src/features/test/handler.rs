use super::model::Test;
use super::model_response::{DeleteTest, TestResponse, NewTest};

use crate::AppState;
use crate::error::AppError;
use axum::{
    Json,
    extract::{Query, State},
};

pub async fn get_all(
    State(state): State<AppState>,
) -> Result<Json<Vec<TestResponse>>, AppError> {
    let tests = state.test_service.get_all().await?;
    Ok(Json(tests))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<NewTest>,
) -> Result<Json<Vec<TestResponse>>, AppError> {
    let tests = state.test_service.create(payload).await?;
    Ok(Json(tests))
}

pub async fn delete(
    State(state): State<AppState>,
    Json(payload): Json<DeleteTest>,
) -> Result<Json<Vec<TestResponse>>, AppError> {
    let tests = state.test_service.delete(payload).await?;
    Ok(Json(tests))
}
