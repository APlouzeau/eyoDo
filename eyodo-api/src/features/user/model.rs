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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            name: user.name,
        }
    }
}
