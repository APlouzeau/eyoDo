use sqlx::SqlitePool;

use crate::features::todo::model::TaskFilter;
use crate::features::todo::model::TaskQueryParams;

use super::model::CreateTaskToDo;
use super::model::Todo;

#[derive(Clone)]
pub struct SqliteTodoRepository {
    pub pool: SqlitePool,
}

impl TodoRepository for SqliteTodoRepository {
    async fn get_all(&self, filter: Option<TaskFilter>) -> Result<Vec<Todo>, sqlx::Error> {
        if filter.is_some() {
            let filter = filter.unwrap();
            match filter {
                TaskFilter::Completed => {
                    return sqlx::query_as::<_, Todo>(
                        r#"
                        SELECT *
                        FROM todos
                        WHERE completed_at IS NOT NULL
                        "#,
                    )
                    .fetch_all(&self.pool)
                    .await;
                }
                TaskFilter::InProgress => {
                    return sqlx::query_as::<_, Todo>(
                        r#"
                        SELECT *
                        FROM todos
                        WHERE completed_at IS NULL
                        "#,
                    )
                    .fetch_all(&self.pool)
                    .await;
                }
            }
        }
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
            RETURNING id, title, description, due_date, assigned_to, comments, completed_at
            "#,
        )
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(&todo.due_date)
        .bind(&todo.assigned_to)
        .fetch_one(&self.pool)
        .await
    }

    async fn complete_todo(&self, id: i32) -> Result<Todo, sqlx::Error> {
        sqlx::query_as::<_, Todo>(
            r#"
            UPDATE todos
            SET completed_at = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING id, title, description, due_date, assigned_to, comments, completed_at
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }
}

pub trait TodoRepository {
    async fn get_all(&self, filter: Option<TaskFilter>) -> Result<Vec<Todo>, sqlx::Error>;
    async fn create(&self, todo: CreateTaskToDo) -> Result<Todo, sqlx::Error>;
    async fn complete_todo(&self, id: i32) -> Result<Todo, sqlx::Error>;
}
