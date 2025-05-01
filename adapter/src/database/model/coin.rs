use kernel::model::{
    coin::{Coin, CoinHistory, CoinType},
    id::{CoinId, UserId},
};
use sqlx::types::chrono::{DateTime as SqlxDateTime, Utc as SqlxUtc};

pub struct CoinRow {
    pub coin_id: CoinId,
    pub user_id: UserId,
    pub session_id: Option<String>,
    pub purchased_amount: i32,
    pub used_amount: i32,
    pub coin_type: String,
    pub created_at: SqlxDateTime<SqlxUtc>,
    pub updated_at: SqlxDateTime<SqlxUtc>,
}

impl CoinRow {
    pub fn into_coin(self) -> Coin {
        let coin_type = match self.coin_type.as_str() {
            "PAID" => CoinType::Paid,
            "FREE" => CoinType::Free,
            _ => panic!("Invalid coin type"),
        };

        Coin {
            id: self.coin_id,
            user_id: self.user_id,
            session_id: self.session_id,
            purchased_amount: self.purchased_amount,
            used_amount: self.used_amount,
            coin_type,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    pub fn into_coin_history(self) -> CoinHistory {
        let coin_type = match self.coin_type.as_str() {
            "PAID" => CoinType::Paid,
            "FREE" => CoinType::Free,
            _ => panic!("Invalid coin type"),
        };

        let coin = self.into_coin();
        CoinHistory {
            coin_id: coin.id,
            purchased_amount: coin.purchased_amount,
            used_amount: coin.used_amount,
            coin_type,
            created_at: coin.created_at,
            remaining_days: coin.remaining_days(),
        }
    }
}
