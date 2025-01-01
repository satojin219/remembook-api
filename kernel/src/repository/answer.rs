use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::answer::event::CreateAnswer;

#[async_trait]
pub trait AnswerRepository: Send + Sync {
    async fn create_answer(&self, event: CreateAnswer) -> AppResult<()>;
}
