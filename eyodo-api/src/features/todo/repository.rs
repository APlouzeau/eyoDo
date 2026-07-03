use sqlx::PgPool;

use crate::features::todo::model::TaskFilter;

use super::model::CreateTaskToDo;
use super::model::Todo;

#[derive(Clone)]
pub struct PostgresTodoRepository {
    pub pool: PgPool,
}

impl TodoRepository for PostgresTodoRepository {
    async fn get_all(&self, filter: Option<TaskFilter>) -> Result<Vec<Todo>, sqlx::Error> {
        let mut query = "SELECT * FROM todos".to_string();

        match filter {
            Some(TaskFilter::Completed) => {
                query += " WHERE completed_at IS NOT NULL";
            }
            Some(TaskFilter::InProgress) => {
                query += " WHERE completed_at IS NULL";
            }
            None => {}
        }

        dbg!(
            sqlx::query_as::<_, Todo>(&query)
                .fetch_all(&self.pool)
                .await
        )
    }

    async fn create(&self, todo: CreateTaskToDo) -> Result<Todo, sqlx::Error> {
        sqlx::query_as::<_, Todo>(
            r#"
            INSERT INTO todos (title, description, due_date, creator_id, owner_user_id, owner_group_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, title, description, due_date, creator_id, owner_user_id, owner_group_id, created_at
            "#,
        )
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(&todo.due_date)
        .bind(&todo.creator_id)
        .bind(&todo.owner_user_id)
        .bind(&todo.owner_group_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn complete_todo(&self, id: i32) -> Result<Todo, sqlx::Error> {
        sqlx::query_as::<_, Todo>(
            r#"
            UPDATE todos
            SET completed_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
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
