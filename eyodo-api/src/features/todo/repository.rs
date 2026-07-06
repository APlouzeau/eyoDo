use super::model::TaskFilter;
use sqlx::PgPool;

use super::model::{NewTodo, Todo};
use super::model_joined::TodoDetail;

#[derive(Clone)]
pub struct PostgresTodoRepository {
    pub pool: PgPool,
}

impl TodoRepository for PostgresTodoRepository {
    async fn get_all(&self, filter: Option<TaskFilter>) -> Result<Vec<TodoDetail>, sqlx::Error> {
        let mut query = "
        SELECT t.id, t.title, t.description, t.due_date, t.completed_at, t.created_at, 
               t.creator_id, u.name AS creator_name,
               t.owner_user_id, ow.name AS owner_name,
               t.owner_group_id, g.name AS owner_group_name 
        FROM todos t
        LEFT JOIN users u ON t.creator_id = u.id
        LEFT JOIN users ow ON t.owner_user_id = ow.id
        LEFT JOIN group_ g ON t.owner_group_id = g.id"
            .to_string();

        match filter {
            Some(TaskFilter::Completed) => {
                query += " WHERE completed_at IS NOT NULL";
            }
            Some(TaskFilter::InProgress) => {
                query += " WHERE completed_at IS NULL";
            }
            None => {}
        }

        let todos = sqlx::query_as::<_, TodoDetail>(&query)
            .fetch_all(&self.pool)
            .await?;
        Ok(todos)
    }

    async fn create(&self, todo: NewTodo) -> Result<Todo, sqlx::Error> {
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
    fn get_all(
        &self,
        filter: Option<TaskFilter>,
    ) -> impl std::future::Future<Output = Result<Vec<TodoDetail>, sqlx::Error>> + Send;
    fn create(
        &self,
        todo: NewTodo,
    ) -> impl std::future::Future<Output = Result<Todo, sqlx::Error>> + Send;
    fn complete_todo(
        &self,
        id: i32,
    ) -> impl std::future::Future<Output = Result<Todo, sqlx::Error>> + Send;
}
