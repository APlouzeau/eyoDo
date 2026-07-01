use axum::{
    Router,
    routing::{get, post},
};

use super::handler;

pub fn routes() -> Router {
    Router::new()
        .route("/tasks", get(handler::get_todos))
        .route("/tasks", post(handler::create_todo))
}
