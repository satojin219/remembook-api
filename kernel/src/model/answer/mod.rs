use super::id::AnswerId;

pub mod event;

#[derive(Debug, PartialEq, Eq)]
pub struct UserAnswer {
    pub id: AnswerId,
    pub body: String,
    pub score: i32,
}
