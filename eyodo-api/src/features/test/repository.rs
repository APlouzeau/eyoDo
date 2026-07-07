use super::model::Test;
use super::model_joined::TestDetails;

impl TestRepository for sqlx::PgPool {
    fn get_all(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<TestDetails>, sqlx::Error>> + Send {
        let pool = self.clone();
        async move {
            let tests = sqlx::query_as!(TestDetails, "SELECT * FROM tests")
                .fetch_all(&pool)
                .await?;
            Ok(tests)
        }
    }

    fn create(
        &self,
        new_test: NewTest,
    ) -> impl std::future::Future<Output = Result<Test, sqlx::Error>> + Send {
        let pool = self.clone();
        async move {
            let new_test = sqlx::query_as!(
                Test,
                "INSERT INTO tests 
                VALUES  
                RETURNING *",
            )
            .fetch_one(&pool)
            .await?;
            Ok(new_test)
        }
    }

    fn delete(
        &self,
        test_id: i32,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send {
        let pool = self.clone();
        async move {
            sqlx::query("DELETE FROM tests WHERE id = $1")
                .bind(test_id)
                .execute(&pool)
                .await?;
            Ok(())
        }
    }
}

pub trait TestRepository {
    fn get_all(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<TestDetails>, sqlx::Error>> + Send;
    fn create(
        &self,
        new_test: NewTest,
    ) -> impl std::future::Future<Output = Result<Test, sqlx::Error>> + Send;
    fn delete(
        &self,
        test_id: i32,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send;
}
