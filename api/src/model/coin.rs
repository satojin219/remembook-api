use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateCoinRequest {
    #[garde(length(min = 1))]
    pub session_id: String,
    #[garde(skip)]
    pub amount: i32,
    #[garde(custom(validate_coin_type))]
    pub coin_type: String,
}

fn validate_coin_type(value: &str, _: &()) -> garde::Result {
    if value != "PAID" && value != "FREE" {
        return Err(garde::Error::new(
            "coin_type must be either 'PAID' or 'FREE'",
        ));
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConsumeCoinRequest {
    #[garde(skip)]
    pub amount: i32,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinHistoryResponse {
    pub coin_id: String,
    pub purchased_amount: i32,
    pub used_amount: i32,
    pub coin_type: String,
    pub created_at: String,
    pub remaining_days: i64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidCoinsResponse {
    pub total_amount: i32,
}
