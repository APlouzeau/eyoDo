use crate::features::user::model::GetUsers;
use crate::features::user::model::User;

use super::model::CreateUser;
use super::repository::UserRepository;

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    pub repository: R,
}

// Les méthodes du service
impl<R: UserRepository> UserService<R> {
    pub async fn get_all(&self) -> Result<Vec<GetUsers>, sqlx::Error> {
        self.repository.get_all().await // délègue au repo
    }

    pub async fn create(&self, user: CreateUser) -> Result<User, sqlx::Error> {
        self.repository.create(user).await // délègue au repo
    }
}
