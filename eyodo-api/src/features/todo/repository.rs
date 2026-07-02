use super::model::CreateTaskToDo;
use super::model::Todo;

pub trait TodoRepository {
    async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error>;
    async fn create(&self, todo: CreateTaskToDo) -> Result<Todo, sqlx::Error>;
}
