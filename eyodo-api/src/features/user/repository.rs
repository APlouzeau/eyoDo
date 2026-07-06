use sqlx::PgPool;

use super::model::NewUser;
use super::model::User;
use super::model::UserResponse;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pub pool: PgPool,
}

impl UserRepository for PostgresUserRepository {
    async fn get_all(&self) -> Result<Vec<UserResponse>, sqlx::Error> {
        let users = sqlx::query_as::<_, UserResponse>("SELECT id, name FROM users")
            .fetch_all(&self.pool)
            .await;
        dbg!(&users);
        users
    }

    async fn create(&self, user: NewUser) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (name, password)
            VALUES ($1, $2)
            RETURNING id, name
            "#,
        )
        .bind(&user.name)
        .bind(&user.password)
        .fetch_one(&self.pool)
        .await
    }
}

pub trait UserRepository {
    async fn get_all(&self) -> Result<Vec<UserResponse>, sqlx::Error>;
    async fn create(&self, user: NewUser) -> Result<User, sqlx::Error>;
}
