use super::model::Test;
use super::model_response::TestResponse;
use super::repository::TestRepository;

#[derive(Clone)]
pub struct TestService<R: TestRepository> {
    pub repository: R,
}

impl<R: TestRepository> TestService<R> {
    pub async fn get_all(
        &self,
        filter: Option<TaskFilter>,
    ) -> Result<Vec<TestResponse>, sqlx::Error> {
        let tests = self.repository.get_all(filter).await?;
        Ok(tests.into_iter().map(TestResponse::from).collect())
    }

    pub async fn create(
        &self,
        new_test: NewTest,
    ) -> Result<Vec<TestResponse>, sqlx::Error> {
        self.repository.create(new_test).await?;
        self.get_all(None).await
    }

    pub async fn delete(&self, id: i32) -> Result<Vec<TestResponse>, sqlx::Error> {
        self.repository.delete(id).await?;
        self.get_all(None).await
    }
}
