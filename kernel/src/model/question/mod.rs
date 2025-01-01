use super::id::QuestionId;

pub mod event;

#[derive(Debug, PartialEq, Eq)]
pub struct Question {
    pub id: QuestionId,
    pub body: String,
}
