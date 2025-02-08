use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::{
    id::MemoId,
    memo::event::{CreateMemo, DeleteMemo, UpdateMemo},
};

#[async_trait]
pub trait MemoRepository: Send + Sync {
    async fn get_by_id(&self, memo_id: MemoId) -> AppResult<Option<String>>;
    async fn create_memo(&self, event: CreateMemo) -> AppResult<MemoId>;
    async fn update_memo(&self, event: UpdateMemo) -> AppResult<()>;
    async fn delete_memo(&self, event: DeleteMemo) -> AppResult<()>;
}
