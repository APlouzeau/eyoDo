use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use super::model_joined::TodoDetail;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TodoResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub due_date: Option<NaiveDate>,
    pub completed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub creator_id: i32,
    pub creator_name: String,
    pub owner_user_id: Option<i32>,
    pub owner_name: Option<String>,
    pub owner_group_id: Option<i32>,
    pub owner_group_name: Option<String>,
}

impl From<TodoDetail> for TodoResponse {
    fn from(todo_detail: TodoDetail) -> Self {
        TodoResponse {
            id: todo_detail.id,
            title: todo_detail.title,
            description: todo_detail.description,
            due_date: todo_detail.due_date,
            completed_at: todo_detail.completed_at,
            created_at: todo_detail.created_at,
            creator_id: todo_detail.creator_id,
            creator_name: todo_detail.creator_name,
            owner_user_id: todo_detail.owner_user_id,
            owner_name: todo_detail.owner_name,
            owner_group_id: todo_detail.owner_group_id,
            owner_group_name: todo_detail.owner_group_name,
        }
    }
}
