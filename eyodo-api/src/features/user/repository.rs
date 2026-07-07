use sqlx::PgPool;

use super::model::{NewUser, User, UserDetail};

#[derive(Clone)]
pub struct PostgresUserRepository {
    pub pool: PgPool,
}

impl UserRepository for PostgresUserRepository {
    async fn get_all(&self) -> Result<Vec<UserDetail>, sqlx::Error> {
        let users = sqlx::query_as!(UserDetail, "SELECT id, name FROM users")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    async fn create(&self, user: NewUser) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, password)
            VALUES ($1, $2)
            RETURNING id, name, password
            "#,
            user.name,
            user.password
        )
        .fetch_one(&self.pool)
        .await
    }
}

pub trait UserRepository {
    fn get_all(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<UserDetail>, sqlx::Error>> + Send;
    fn create(
        &self,
        user: NewUser,
    ) -> impl std::future::Future<Output = Result<User, sqlx::Error>> + Send;
}
