use garde::Validate;
use kernel::model::{id::UserId, user::event::UpdateCoin};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddCoinRequest {
    #[garde(range(min = 1))]
    pub amount: i32,
}

pub struct AddCoinRequestWithIds(pub UserId, pub AddCoinRequest);

impl From<AddCoinRequestWithIds> for UpdateCoin {
    fn from(value: AddCoinRequestWithIds) -> Self {
        let AddCoinRequestWithIds(user_id, AddCoinRequest { amount }) = value;
        UpdateCoin { user_id, amount }
    }
}
