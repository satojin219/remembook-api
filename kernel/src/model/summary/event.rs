use crate::model::id::{BookId, SummaryId, UserId};

#[derive(Debug)]
pub struct CreateSummary {
    pub user_id: UserId,
    pub book_id: BookId,
    pub summary_text: String,
}

#[derive(Debug)]
pub struct UpdateSummary {
    pub summary_id: SummaryId,
    pub summary_text: String,
}

#[derive(Debug)]
pub struct DeleteSummary {
    pub summary_id: SummaryId,
}
