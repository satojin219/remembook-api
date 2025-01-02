use derive_new::new;
use garde::Validate;
use kernel::model::{
    id::{BookId, QuestionId, SummaryId, UserId},
    question::event::CreateQuestion,
    summary::event::CreateSummary,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct CreateSummaryRequest {
    #[garde(length(min = 1))]
    pub body: String,
}

#[derive(new)]
pub struct CreateSummaryRequestWithIds(pub UserId, pub BookId, pub CreateSummaryRequest);

impl From<CreateSummaryRequestWithIds> for CreateSummary {
    fn from(value: CreateSummaryRequestWithIds) -> Self {
        let CreateSummaryRequestWithIds(user_id, book_id, CreateSummaryRequest { body }) = value;
        CreateSummary {
            book_id,
            user_id,
            summary_text: body,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateQuestionRequest {
    #[garde(length(min = 1))]
    pub body: String,
}

#[derive(new)]
pub struct CreateQuestionRequestWithIds(
    pub UserId,
    pub BookId,
    pub SummaryId,
    pub CreateQuestionRequest,
);

impl From<CreateQuestionRequestWithIds> for CreateQuestion {
    fn from(value: CreateQuestionRequestWithIds) -> Self {
        let CreateQuestionRequestWithIds(
            user_id,
            book_id,
            summary_id,
            CreateQuestionRequest { body },
        ) = value;
        CreateQuestion {
            user_id,
            book_id,
            summary_id,
            question_text: body,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSummaryRequest {
    #[garde(length(min = 1))]
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionResponse {
    pub question_id: QuestionId,
    pub body: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserAnswerRequest {
    #[garde(length(min = 1))]
    pub user_answer: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAnswerResponse {
    pub score: i32,
    pub user_answer: String,
    pub summary: String,
}
