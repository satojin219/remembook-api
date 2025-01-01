use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        id::SummaryId,
        summary::event::{CreateSummary, DeleteSummary, UpdateSummary},
    },
    repository::summary::SummaryRepository,
};
use shared::error::{AppError, AppResult};

use crate::database::ConnectionPool;

#[derive(new)]
pub struct SummaryRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl SummaryRepository for SummaryRepositoryImpl {
    async fn create_summary(&self, event: CreateSummary) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO summaries(summary_text)
                VALUES($1)
            "#,
            event.summary_text
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }
    async fn update_summary(&self, event: UpdateSummary) -> AppResult<()> {
        let row = sqlx::query!(
            r#"
                UPDATE summaries SET summary_text = $1
                WHERE summary_id = $2
            "#,
            event.summary_text,
            event.summary_id as _,
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if row.rows_affected() < 1 {
            return Err(AppError::EntityNotFound(
                "Specified summary not found".into(),
            ));
        }

        Ok(())
    }
    async fn delete_summary(&self, event: DeleteSummary) -> AppResult<()> {
        let row = sqlx::query!(
            r#"
                DELETE FROM summaries
                WHERE summary_id = $1
            "#,
            event.summary_id as _,
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if row.rows_affected() < 1 {
            return Err(AppError::EntityNotFound(
                "Specified summary not found".into(),
            ));
        }

        Ok(())
    }
}
