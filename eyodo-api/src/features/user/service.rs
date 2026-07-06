use crate::features::user::model::UserResponse;

use super::model::NewUser;
use super::repository::UserRepository;

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    pub repository: R,
}

// Les méthodes du service
impl<R: UserRepository> UserService<R> {
    pub async fn get_all(&self) -> Result<Vec<UserResponse>, sqlx::Error> {
        self.repository.get_all().await // délègue au repo
    }

    pub async fn create(&self, user: NewUser) -> Result<UserResponse, sqlx::Error> {
        let user = self.repository.create(user).await?;
        Ok(user.into()) // délègue au repo
    }
}
