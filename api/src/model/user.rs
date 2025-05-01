use kernel::model::id::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub user_id: UserId,
    pub coins: i32,
    pub logined_at: String,
}
