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
            SELECT id, title, description, date_to_finish, status, assigned_to, comments, completed_at
            FROM todos
            "#,
        ).fetch_all(&self.pool).
        await
    }

    async fn create(&self, todo: CreateTaskToDo) -> Result<Todo, sqlx::Error> {
        todo!()
    }
}

pub trait TodoRepository {
    async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error>;
    async fn create(&self, todo: CreateTaskToDo) -> Result<Todo, sqlx::Error>;
}
