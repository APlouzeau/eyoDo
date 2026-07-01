use axum::Router;
use tokio::net::TcpListener;

use crate::features::todo::router as todo_router;

mod db;
mod features;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/api", todo_router::routes());
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
