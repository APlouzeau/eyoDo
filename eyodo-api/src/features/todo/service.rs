use crate::features::todo::model::{TaskFilter, TaskQueryParams};

use super::model::{CreateTaskToDo, Todo};
use super::repository::TodoRepository;

#[derive(Clone)]
pub struct TodoService<R: TodoRepository> {
    pub repository: R,
}

// Les méthodes du service
impl<R: TodoRepository> TodoService<R> {
    pub async fn get_all(&self, filter: Option<TaskFilter>) -> Result<Vec<Todo>, sqlx::Error> {
        self.repository.get_all(filter).await // délègue au repo
    }

    pub async fn create(&self, todo: CreateTaskToDo) -> Result<Todo, sqlx::Error> {
        self.repository.create(todo).await // délègue au repo
    }

    pub async fn complete_todo(&self, id: i32) -> Result<Todo, sqlx::Error> {
        self.repository.complete_todo(id).await // délègue au repo
    }
}
