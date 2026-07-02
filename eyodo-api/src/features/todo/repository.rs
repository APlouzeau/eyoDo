use sqlx::SqlitePool;

use super::model::CreateTaskToDo;
use super::model::Todo;

#[derive(Clone)]
pub struct SqliteTodoRepository {
    pub pool: SqlitePool,
}

impl TodoRepository for SqliteTodoRepository {
    async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        sqlx::query_as::<_, Todo>(
            r#"
            SELECT *
            FROM todos
            "#,
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn create(&self, todo: CreateTaskToDo) -> Result<Todo, sqlx::Error> {
        sqlx::query_as::<_, Todo>(
            r#"
            INSERT INTO todos (title, description, due_date, assigned_to)
            VALUES (?, ?, ?, ?)
            RETURNING id, title, description, due_date, status, assigned_to, comments, completed_at
            "#,
        )
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(&todo.due_date)
        .bind(&todo.assigned_to)
        .fetch_one(&self.pool)
        .await
    }
}

pub trait TodoRepository {
    async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error>;
    async fn create(&self, todo: CreateTaskToDo) -> Result<Todo, sqlx::Error>;
}
