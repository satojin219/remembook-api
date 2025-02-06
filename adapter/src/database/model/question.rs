use kernel::model::{
    id::{MemoId, QuestionId},
    question::Question,
};

pub struct QuestionRow {
    pub question_id: QuestionId,
    pub question_text: String,
    pub memo_id: MemoId,
}

impl From<QuestionRow> for Question {
    fn from(value: QuestionRow) -> Self {
        let QuestionRow {
            question_id,
            question_text,
            memo_id,
        } = value;
        Question {
            id: question_id,
            body: question_text,
            memo_id,
        }
    }
}
