use kernel::model::{id::SummaryId, summary::Summary};
pub struct SummaryRow {
    pub summary_id: SummaryId,
    pub summary_text: String,
}

impl SummaryRow {
    pub fn into_summary(self) -> Summary {
        Summary {
            id: self.summary_id,
            body: self.summary_text,
        }
    }
}
