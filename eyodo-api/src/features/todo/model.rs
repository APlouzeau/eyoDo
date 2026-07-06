use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub due_date: Option<NaiveDate>,
    pub completed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub creator_id: i32,
    pub owner_user_id: Option<i32>,
    pub owner_group_id: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewToDo {
    pub title: String,
    pub description: String,
    pub due_date: Option<NaiveDate>,
    pub creator_id: i32,
    pub owner_user_id: Option<i32>,
    pub owner_group_id: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct TaskQueryParams {
    pub filter: Option<TaskFilter>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskFilter {
    Completed,
    InProgress,
}

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
