use crate::model::id::SummaryId;

#[derive(Debug)]
pub struct CreateQuestion {
    pub summary_id: SummaryId,
    pub question_text: String,
}

#[derive(Debug)]
pub struct UpdateQuestion {
    pub summary_id: SummaryId,
    pub question_text: String,
}
