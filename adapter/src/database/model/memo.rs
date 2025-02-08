use kernel::model::{id::MemoId, memo::Memo};

pub struct MemoRow {
    pub memo_id: MemoId,
    pub memo_text: String,
}

impl MemoRow {
    pub fn into_memo(self) -> Memo {
        Memo {
            id: self.memo_id,
            body: self.memo_text,
        }
    }
}
