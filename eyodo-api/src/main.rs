use axum::Router;
use tokio::net::TcpListener;

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
            date_to_finish DATETIME DEFAULT NULL,
            assigned_to VARCHAR(255) default NULL,
            status TEXT CHECK(status IN ('en attente', 'en cours', 'terminé')) DEFAULT 'en attente',
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

    let app = Router::new()
        .nest("/api", todo_router::routes())
        .with_state(state);
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
