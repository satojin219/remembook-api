use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::{
    coin::{
        event::{ConsumeCoin, CreateCoin},
        CoinHistory,
    },
    id::{CoinId, UserId},
};

#[async_trait]
pub trait CoinRepository: Send + Sync {
    async fn get_valid_coins_by_user_id(&self, user_id: UserId) -> AppResult<i32>;
    async fn get_purchase_history_by_user_id(&self, user_id: UserId)
        -> AppResult<Vec<CoinHistory>>;
    async fn create_coin(&self, event: CreateCoin) -> AppResult<CoinId>;
    async fn consume_coin(&self, event: ConsumeCoin) -> AppResult<()>;
}
