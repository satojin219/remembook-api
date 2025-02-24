use garde::Validate;
use kernel::model::{id::UserId, user::User};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddCoinRequest {
    #[garde(range(min = 1))]
    pub amount: i32,
    #[garde(length(min = 1))]
    pub session_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub user_id: UserId,
    pub coins: i32,
    pub logined_at: String,
}

impl From<User> for UserResponse {
    fn from(user_value: User) -> Self {
        let User {
            id,
            coins,
            logined_at,
            ..
        } = user_value;
        Self {
            user_id: id,
            coins,
            logined_at: logined_at.to_rfc3339(),
        }
    }
}
