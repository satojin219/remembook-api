use kernel::model::{id::QuestionId, question::Question};
pub struct QuestionRow {
    pub question_id: QuestionId,
    pub question_text: String,
}

impl From<QuestionRow> for Question {
    fn from(value: QuestionRow) -> Self {
        let QuestionRow {
            question_id,
            question_text,
        } = value;
        Question {
            id: question_id,
            body: question_text,
        }
    }
}
