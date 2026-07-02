use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::features::todo::repository::SqliteTodoRepository;
use crate::features::todo::router as todo_router;
use crate::features::todo::service::TodoService;

mod db;
mod features;

#[derive(Clone)]
pub struct AppState {
    pub todo_service: TodoService<SqliteTodoRepository>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // Charger les variables d'environnement depuis le fichier .env
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title VARCHAR(255) NOT NULL,
            description TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            due_date DATETIME DEFAULT NULL,
            status TEXT CHECK(status IN ('en attente', 'en cours', 'terminé')) DEFAULT 'en attente',
            assigned_to VARCHAR(255) default NULL,
            comments TEXT,
            completed_at DATETIME default NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create todos table");

    let state = AppState {
        todo_service: TodoService {
            repository: SqliteTodoRepository { pool: pool.clone() },
        },
    };

    let cors = CorsLayer::new()
        .allow_origin(
            std::env::var("URL_CORS")
                .unwrap_or_else(|_| "http://localhost:3000".to_string())
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", todo_router::routes())
        .layer(cors)
        .with_state(state);
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
