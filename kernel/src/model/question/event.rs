use crate::model::id::{BookId, MemoId, UserId};

#[derive(Debug)]
pub struct CreateQuestion {
    pub user_id: UserId,
    pub book_id: BookId,
    pub memo_id: MemoId,
    pub question_text: String,
}

#[derive(Debug)]
pub struct UpdateQuestion {
    pub memo_id: MemoId,
    pub question_text: String,
}
