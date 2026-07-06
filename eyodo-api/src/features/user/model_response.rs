use crate::features::user::model::UserDetail;

use super::model::User;
use serde::{Deserialize, Serialize};

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

impl From<UserDetail> for UserResponse {
    fn from(user: UserDetail) -> Self {
        UserResponse {
            id: user.id,
            name: user.name,
        }
    }
}
