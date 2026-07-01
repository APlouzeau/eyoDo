use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub date_to_finish: Option<String>,
    pub status: Option<String>,
    pub assigned_to: Option<String>,
    pub comments: Vec<String>,
    pub completed_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTaskToDo {
    pub title: String,
    pub description: String,
    pub date_to_finish: Option<String>,
    pub assigned_to: Option<String>,
}
