use crate::model::id::{BookId, MemoId, UserId};

#[derive(Debug)]
pub struct CreateMemo {
    pub user_id: UserId,
    pub book_id: BookId,
    pub memo_text: String,
}

#[derive(Debug)]
pub struct UpdateMemo {
    pub memo_id: MemoId,
    pub memo_text: String,
}

#[derive(Debug)]
pub struct DeleteMemo {
    pub memo_id: MemoId,
}
