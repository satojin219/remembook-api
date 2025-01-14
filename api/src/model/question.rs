use kernel::model::{
    id::{QuestionId, SummaryId},
    question::Question,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionResponse {
    pub id: QuestionId,
    pub body: String,
    pub summary_id: SummaryId,
}

impl From<Question> for QuestionResponse {
    fn from(value: Question) -> Self {
        let Question { id, body,summary_id } = value;
        Self { id, body, summary_id }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionsResponse {
    pub questions: Vec<QuestionResponse>,
}
