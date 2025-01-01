use crate::model::id::QuestionId;

#[derive(Debug)]
pub struct CreateAnswer {
    pub question_id: QuestionId,
    pub answer_text: String,
    pub score: i32,
}
