use super::id::MemoId;

pub mod event;

#[derive(Debug, PartialEq, Eq)]
pub struct Memo {
    pub id: MemoId,
    pub body: String,
}
