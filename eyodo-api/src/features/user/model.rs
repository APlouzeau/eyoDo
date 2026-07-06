use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct NewUser {
    pub name: String,
    pub password: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserDetail {
    pub id: i32,
    pub name: String,
}
