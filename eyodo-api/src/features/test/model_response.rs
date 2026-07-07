use serde::{Deserialize, Serialize};

use super::model_joined::TestDetails;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TestResponse {
    pub id: i32,
}

impl From<TestDetails> for TestResponse {
    fn from(details: TestDetails) -> Self {
        TestResponse { id: details.id }
    }
}
