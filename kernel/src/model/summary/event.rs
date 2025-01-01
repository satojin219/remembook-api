use crate::model::id::SummaryId;

#[derive(Debug)]
pub struct CreateSummary {
    pub summary_id: SummaryId,
    pub summary_text: String,
}

#[derive(Debug)]
pub struct UpdateSummary {
    pub summary_id: SummaryId,
    pub summary_text: String,
}

#[derive(Debug)]
pub struct DeleteSummary {
    pub summary_id: SummaryId,
}
