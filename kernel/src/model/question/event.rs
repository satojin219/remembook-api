use crate::model::id::{BookId, SummaryId, UserId};

#[derive(Debug)]
pub struct CreateQuestion {
    pub user_id: UserId,
    pub book_id: BookId,
    pub summary_id: SummaryId,
    pub question_text: String,
}

#[derive(Debug)]
pub struct UpdateQuestion {
    pub summary_id: SummaryId,
    pub question_text: String,
}
