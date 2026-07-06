use super::model::TaskFilter;
use super::model_joined::TodoDetail;
use super::model_response::TodoResponse;

use super::model::NewTodo;
use super::repository::TodoRepository;

#[derive(Clone)]
pub struct TodoService<R: TodoRepository> {
    pub repository: R,
}

// Les méthodes du service
impl<R: TodoRepository> TodoService<R> {
    pub async fn get_all(
        &self,
        filter: Option<TaskFilter>,
    ) -> Result<Vec<TodoResponse>, sqlx::Error> {
        let todos = self.repository.get_all(filter).await?;
        Ok(todos.into_iter().map(TodoResponse::from).collect())
    }

    pub async fn create(&self, todo: NewTodo) -> Result<Vec<TodoResponse>, sqlx::Error> {
        self.repository.create(todo).await?;
        self.get_all(None).await
    }

    pub async fn complete_todo(&self, id: i32) -> Result<Vec<TodoResponse>, sqlx::Error> {
        self.repository.complete_todo(id).await?;
        self.get_all(None).await
    }
}
