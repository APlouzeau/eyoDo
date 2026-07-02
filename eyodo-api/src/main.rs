use axum::Router;
use tokio::net::TcpListener;

use crate::features::todo::router as todo_router;

mod db;
mod features;

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

    let app = Router::new().nest("/api", todo_router::routes());
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
