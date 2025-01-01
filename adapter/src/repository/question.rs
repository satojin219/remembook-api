use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        id::SummaryId,
        question::{
            event::{CreateQuestion, UpdateQuestion},
            Question,
        },
    },
    repository::question::QuestionRepository,
};
use shared::error::{AppError, AppResult};

use crate::database::{model::question::QuestionRow, ConnectionPool};

#[derive(new)]
pub struct QuestionRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl QuestionRepository for QuestionRepositoryImpl {
    async fn get_by_summary_id(&self, summary_id: SummaryId) -> AppResult<Option<Question>> {
        let row: Option<QuestionRow> = sqlx::query_as!(
            QuestionRow,
            r#"
        SELECT question_id, question_text FROM questions WHERE summary_id = $1
            "#,
            summary_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        match row {
            Some(r) => Ok(Some(r.into_question())),
            None => Ok(None),
        }
    }
    async fn create_question(&self, event: CreateQuestion) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO questions(question_text)
                VALUES($1)
            "#,
            event.question_text
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }
    async fn update_question(&self, event: UpdateQuestion) -> AppResult<()> {
        sqlx::query!(
            r#"
                UPDATE questions SET question_text = $1
            "#,
            event.question_text
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }
}
