use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tasks", get(handler::get_todos))
        .route("/tasks", post(handler::create_todo))
        .route("/tasks/{id}/complete", post(handler::complete_todo))
}
