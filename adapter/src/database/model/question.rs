use kernel::model::{id::QuestionId, question::Question};
pub struct QuestionRow {
    pub question_id: QuestionId,
    pub question_text: String,
}

impl QuestionRow {
    pub fn into_question(self) -> Question {
        Question {
            id: self.question_id,
            body: self.question_text,
        }
    }
}
