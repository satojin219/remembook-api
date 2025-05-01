use crate::model::{coin::CoinType, id::UserId};

#[derive(Debug)]
pub struct CreateCoin {
    pub user_id: UserId,
    pub session_id: Option<String>,
    pub amount: i32,
    pub coin_type: CoinType,
}

#[derive(Debug)]
pub struct ConsumeCoin {
    pub user_id: UserId,
    pub amount: i32,
}

#[derive(Debug)]
pub struct GetValidCoins {
    pub user_id: UserId,
}

#[derive(Debug)]
pub struct GetPurchaseHistory {
    pub user_id: UserId,
}
