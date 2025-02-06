use derive_new::new;
use garde::Validate;
use kernel::model::{
    id::{BookId, MemoId, QuestionId, UserId},
    memo::event::CreateMemo,
    question::event::CreateQuestion,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateMemoRequest {
    #[garde(length(min = 1))]
    pub body: String,
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(length(min = 1))]
    pub author: Vec<String>,
    #[garde(url)]
    pub image_url: String,
    #[garde(length(min = 1))]
    pub google_books_id: String,
}

#[derive(new)]
pub struct CreateMemoRequestWithIds(pub UserId, pub BookId, pub CreateMemoRequest);

impl From<CreateMemoRequestWithIds> for CreateMemo {
    fn from(value: CreateMemoRequestWithIds) -> Self {
        let CreateMemoRequestWithIds(
            user_id,
            book_id,
            CreateMemoRequest {
                body,
                title: _,
                author: _,
                image_url: _,
                google_books_id: _,
            },
        ) = value;
        CreateMemo {
            book_id,
            user_id,
            memo_text: body,
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
    pub MemoId,
    pub CreateQuestionRequest,
);

impl From<CreateQuestionRequestWithIds> for CreateQuestion {
    fn from(value: CreateQuestionRequestWithIds) -> Self {
        let CreateQuestionRequestWithIds(user_id, book_id, memo_id, CreateQuestionRequest { body }) =
            value;
        CreateQuestion {
            user_id,
            book_id,
            memo_id,
            question_text: body,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMemoRequest {
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
    pub memo: String,
}
