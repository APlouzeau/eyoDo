use super::model::NewUser;
use super::model_response::UserResponse;

use super::repository::UserRepository;

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    pub repository: R,
}

// Les méthodes du service
impl<R: UserRepository> UserService<R> {
    pub async fn get_all(&self) -> Result<Vec<UserResponse>, sqlx::Error> {
        let users = self.repository.get_all().await?; // délègue au repo
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    pub async fn create(&self, user: NewUser) -> Result<UserResponse, sqlx::Error> {
        let user = self.repository.create(user).await?;
        Ok(UserResponse::from(user))
    }
}
