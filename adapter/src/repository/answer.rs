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
                INSERT INTO user_answers(answer_text,score,question_id,user_id)
                VALUES($1,$2,$3,$4)
            "#,
            event.answer_text,
            event.score,
            event.question_id as _,
            event.user_id as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }
}
