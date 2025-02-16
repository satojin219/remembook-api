use garde::Validate;
use kernel::model::id::UserId;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddCoinRequest {
    #[garde(skip)]
    pub user_id: UserId,
    #[garde(range(min = 1))]
    pub amount: i32,
    #[garde(length(min = 1))]
    pub session_id: String,
}
