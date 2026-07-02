use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub due_date: Option<String>,
    pub status: Option<String>,
    pub assigned_to: Option<String>,
    pub comments: Option<String>,
    pub completed_at: Option<String>,
}
#[derive(Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskToDo {
    pub title: String,
    pub description: String,
    pub due_date: Option<String>,
    pub assigned_to: Option<String>,
}
