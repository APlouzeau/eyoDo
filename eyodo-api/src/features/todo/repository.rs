use super::model::Todo;

pub trait TodoRepository {
    fn get_all_todos(&self) -> Vec<Todo>;
    fn create_todo(&self, todo: Todo);
}
