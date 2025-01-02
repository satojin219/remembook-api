use kernel::model::{id::QuestionId, question::Question};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionResponse {
    pub id: QuestionId,
    pub body: String,
}

impl From<Question> for QuestionResponse {
    fn from(value: Question) -> Self {
        let Question { id, body } = value;
        Self { id, body }
    }
}
