use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::{
    id::UserId,
    user::{
        event::{AddPurchaseHistory, CreateUser, DeleteUser, UpdateCoin, UpdateUserPassword},
        User,
    },
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_current_user(&self, current_user_id: UserId) -> AppResult<Option<User>>;
    async fn create(&self, event: CreateUser) -> AppResult<User>;
    async fn update_password(&self, event: UpdateUserPassword) -> AppResult<()>;
    async fn delete(&self, event: DeleteUser) -> AppResult<()>;
    async fn update_coin(&self, event: UpdateCoin) -> AppResult<()>;
    async fn add_purchase_history(&self, event: AddPurchaseHistory) -> AppResult<()>;
    async fn update_logined_at(&self, user_id: UserId) -> AppResult<()>;
}
