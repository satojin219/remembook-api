use kernel::model::{answer::UserAnswer, id::AnswerId};
pub struct UserAnswerRow {
    pub answer_id: AnswerId,
    pub answer_text: String,
    pub score: i32,
}

impl UserAnswerRow {
    pub fn into_answer(self) -> UserAnswer {
        UserAnswer {
            id: self.answer_id,
            body: self.answer_text,
            score: self.score,
        }
    }
}
