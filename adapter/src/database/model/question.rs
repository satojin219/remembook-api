use kernel::model::{
    id::{QuestionId, SummaryId},
    question::Question,
};
pub struct QuestionRow {
    pub question_id: QuestionId,
    pub question_text: String,
    pub summary_id: SummaryId,
}

impl From<QuestionRow> for Question {
    fn from(value: QuestionRow) -> Self {
        let QuestionRow {
            question_id,
            question_text,
            summary_id,
        } = value;
        Question {
            id: question_id,
            body: question_text,
            summary_id,
        }
    }
}
