use super::id::SummaryId;

pub mod event;

#[derive(Debug, PartialEq, Eq)]
pub struct Summary {
    pub id: SummaryId,
    pub body: String,
}
