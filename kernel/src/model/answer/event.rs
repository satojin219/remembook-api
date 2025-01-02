use crate::model::id::{QuestionId, UserId};

#[derive(Debug)]
pub struct CreateAnswer {
    pub user_id: UserId,
    pub question_id: QuestionId,
    pub answer_text: String,
    pub score: i32,
}
