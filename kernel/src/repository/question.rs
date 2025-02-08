use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::{
    id::{BookId, MemoId},
    question::{
        event::{CreateQuestion, UpdateQuestion},
        Question,
    },
};

#[async_trait]
pub trait QuestionRepository: Send + Sync {
    async fn get_by_memo_id(&self, memo_id: MemoId) -> AppResult<Option<Question>>;
    async fn get_list_by_book_id(&self, book_id: BookId) -> AppResult<Vec<Question>>;
    async fn create_question(&self, event: CreateQuestion) -> AppResult<()>;
    async fn update_question(&self, event: UpdateQuestion) -> AppResult<()>;
}
