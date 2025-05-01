use async_trait::async_trait;
use derive_new::new;
use sqlx::query_as;

use crate::database::{model::coin::CoinRow, ConnectionPool};

use kernel::{
    model::{
        coin::{
            event::{ConsumeCoin, CreateCoin},
            CoinHistory, CoinType,
        },
        id::{CoinId, UserId},
    },
    repository::coin::CoinRepository,
};
use shared::error::{AppError, AppResult};

#[derive(new)]
pub struct CoinRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl CoinRepository for CoinRepositoryImpl {
    async fn get_valid_coins_by_user_id(&self, user_id: UserId) -> AppResult<i32> {
        let coins: Vec<CoinRow> = query_as!(
            CoinRow,
            r#"
            SELECT 
                coin_id,
                user_id,
                session_id,
                purchased_amount,
                used_amount,
                coin_type,
                created_at,
                updated_at
            FROM coins
            WHERE user_id = $1
            AND created_at + INTERVAL '180 days' > NOW()
            AND purchased_amount > used_amount
            ORDER BY created_at DESC
            "#,
            user_id as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        let total_amount = coins
            .into_iter()
            .map(|c| c.into_coin().remaining_amount() as i32)
            .sum();

        Ok(total_amount)
    }

    async fn get_purchase_history_by_user_id(
        &self,
        user_id: UserId,
    ) -> AppResult<Vec<CoinHistory>> {
        let coins: Vec<CoinRow> = query_as!(
            CoinRow,
            r#"
            SELECT 
                coin_id,
                user_id,
                session_id,
                purchased_amount,
                used_amount,
                coin_type,
                created_at,
                updated_at
            FROM coins
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(coins.into_iter().map(|c| c.into_coin_history()).collect())
    }

    async fn create_coin(&self, event: CreateCoin) -> AppResult<CoinId> {
        let coin_id = CoinId::new();
        let coin_type_str = match event.coin_type {
            CoinType::Paid => "PAID",
            CoinType::Free => "FREE",
        };

        query_as!(
            CoinRow,
            r#"
            INSERT INTO coins (
                coin_id,
                user_id,
                session_id,
                purchased_amount,
                used_amount,
                coin_type
            ) VALUES ($1, $2, $3, $4, 0, $5)
            RETURNING 
                coin_id,
                user_id,
                session_id,
                purchased_amount,
                used_amount,
                coin_type,
                created_at,
                updated_at
            "#,
            coin_id as _,
            event.user_id as _,
            event.session_id,
            event.amount,
            coin_type_str
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(coin_id)
    }

    async fn consume_coin(&self, event: ConsumeCoin) -> AppResult<()> {
        let result = query_as!(
            CoinRow,
            r#"
            UPDATE coins
            SET used_amount = used_amount + $1
            WHERE user_id = $2
            AND created_at + INTERVAL '180 days' > NOW()
            AND purchased_amount - used_amount >= $1
            RETURNING 
                coin_id,
                user_id,
                session_id,
                purchased_amount,
                used_amount,
                coin_type,
                created_at,
                updated_at
            "#,
            event.amount,
            event.user_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        if result.is_none() {
            return Err(AppError::EntityNotFound("Specified coin not found".into()));
        }

        Ok(())
    }
}
