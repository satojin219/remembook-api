use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        id::MemoId,
        memo::event::{CreateMemo, DeleteMemo, UpdateMemo},
    },
    repository::memo::MemoRepository,
};
use shared::error::{AppError, AppResult};

use crate::database::{model::memo::MemoRow, ConnectionPool};

#[derive(new)]
pub struct MemoRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl MemoRepository for MemoRepositoryImpl {
    async fn get_by_id(&self, memo_id: MemoId) -> AppResult<Option<String>> {
        let row: Option<MemoRow> = sqlx::query_as!(
            MemoRow,
            r#"
                SELECT memo_id, memo_text FROM memos
                WHERE memo_id = $1
            "#,
            memo_id as _,
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        match row {
            Some(m) => Ok(Some(m.memo_text.into())),
            None => Ok(None),
        }
    }

    async fn create_memo(&self, event: CreateMemo) -> AppResult<MemoId> {
        let memo_id: MemoId = sqlx::query!(
            r#"
            INSERT INTO memos (memo_text, user_id, book_id)
            VALUES ($1, $2, $3)
            RETURNING memo_id
        "#,
            event.memo_text,
            event.user_id as _,
            event.book_id as _
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?
        .memo_id
        .into();

        Ok(memo_id)
    }

    async fn update_memo(&self, event: UpdateMemo) -> AppResult<()> {
        let row = sqlx::query!(
            r#"
                UPDATE memos SET memo_text = $1
                WHERE memo_id = $2
            "#,
            event.memo_text,
            event.memo_id as _,
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if row.rows_affected() < 1 {
            return Err(AppError::EntityNotFound("Specified memo not found".into()));
        }

        Ok(())
    }

    async fn delete_memo(&self, event: DeleteMemo) -> AppResult<()> {
        let row = sqlx::query!(
            r#"
                DELETE FROM memos
                WHERE memo_id = $1
            "#,
            event.memo_id as _,
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if row.rows_affected() < 1 {
            return Err(AppError::EntityNotFound("Specified memo not found".into()));
        }

        Ok(())
    }
}
