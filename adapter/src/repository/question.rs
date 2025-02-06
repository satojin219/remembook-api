use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        id::{BookId, MemoId},
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
    async fn get_by_memo_id(&self, memo_id: MemoId) -> AppResult<Option<Question>> {
        let row: Option<QuestionRow> = sqlx::query_as!(
            QuestionRow,
            r#"
        SELECT question_id, question_text, memo_id FROM questions WHERE memo_id = $1
            "#,
            memo_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        match row {
            Some(r) => Ok(Some(r.into())),
            None => Ok(None),
        }
    }

    async fn get_list_by_book_id(&self, book_id: BookId) -> AppResult<Vec<Question>> {
        let rows: Vec<QuestionRow> = sqlx::query_as!(
            QuestionRow,
            r#"
        SELECT question_id, question_text, memo_id FROM questions WHERE book_id = $1
            "#,
            book_id as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(rows.into_iter().map(Question::from).collect())
    }

    async fn create_question(&self, event: CreateQuestion) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO questions(question_text, user_id, book_id, memo_id)
                VALUES($1, $2, $3, $4)
            "#,
            event.question_text,
            event.user_id as _,
            event.book_id as _,
            event.memo_id as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }

    async fn update_question(&self, event: UpdateQuestion) -> AppResult<()> {
        let row = sqlx::query!(
            r#"
                UPDATE questions SET question_text = $1
            "#,
            event.question_text
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if row.rows_affected() < 1 {
            return Err(AppError::EntityNotFound(
                "Specified question not found".into(),
            ));
        }
        Ok(())
    }
}
