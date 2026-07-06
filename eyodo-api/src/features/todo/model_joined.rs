use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TodoDetail {
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
