use super::model::{CreateTaskToDo, Todo};
use super::repository::TodoRepository;

pub struct TodoService<R: TodoRepository> {
    pub repository: R,
}

// Les méthodes du service
impl<R: TodoRepository> TodoService<R> {
    pub async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        self.repository.get_all().await // délègue au repo
    }

    pub async fn create(&self, todo: CreateTaskToDo) -> Result<Todo, sqlx::Error> {
        self.repository.create(todo).await // délègue au repo
    }
}
