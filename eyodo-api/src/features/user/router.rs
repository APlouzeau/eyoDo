use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(handler::get_users))
        .route("/users", post(handler::create_user))
}
