use async_trait::async_trait;
use derive_new::new;
use kernel::{model::answer::event::CreateAnswer, repository::answer::AnswerRepository};
use shared::error::{AppError, AppResult};

use crate::database::ConnectionPool;

#[derive(new)]
pub struct AnswerRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl AnswerRepository for AnswerRepositoryImpl {
    async fn create_answer(&self, event: CreateAnswer) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO user_answers(answer_text,score)
                VALUES($1,$2)
            "#,
            event.answer_text,
            event.score
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }
}
