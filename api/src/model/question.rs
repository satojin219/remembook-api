use kernel::model::{
    id::{MemoId, QuestionId},
    question::Question,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionResponse {
    pub id: QuestionId,
    pub body: String,
    pub memo_id: MemoId,
}

impl From<Question> for QuestionResponse {
    fn from(value: Question) -> Self {
        let Question { id, body, memo_id } = value;
        Self { id, body, memo_id }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionsResponse {
    pub questions: Vec<QuestionResponse>,
}
