use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::AppState;

use super::handler;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tests", get(handler::get_all))
        .route("/tests", post(handler::create))
        .route("/tests/{id}", axum::routing::delete(handler::delete))
}
