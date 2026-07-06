use super::model::NewUser;
use super::model_response::UserResponse;
use crate::AppState;
use crate::error::AppError;

use axum::Json;
use axum::extract::State;

pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = state.user_service.get_all().await?;
    Ok(Json(users))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> Result<Json<UserResponse>, AppError> {
    let user = state.user_service.create(payload).await?;
    Ok(Json(user))
}
