use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::summary::event::{CreateSummary, DeleteSummary, UpdateSummary};

#[async_trait]
pub trait SummaryRepository: Send + Sync {
    async fn create_summary(&self, event: CreateSummary) -> AppResult<()>;
    async fn update_summary(&self, event: UpdateSummary) -> AppResult<()>;
    async fn delete_summary(&self, event: DeleteSummary) -> AppResult<()>;
}
