use super::id::{QuestionId, SummaryId};

pub mod event;

#[derive(Debug, PartialEq, Eq)]
pub struct Question {
    pub id: QuestionId,
    pub body: String,
    pub summary_id: SummaryId,
}
