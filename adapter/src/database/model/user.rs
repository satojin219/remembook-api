use kernel::model::{id::UserId, user::User};
use shared::error::AppError;
use sqlx::types::chrono::{DateTime, Utc};

pub struct UserRow {
    pub user_id: UserId,
    pub name: String,
    pub email: String,
    pub coins: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<UserRow> for User {
    type Error = AppError;
    fn try_from(value: UserRow) -> Result<Self, Self::Error> {
        let UserRow {
            user_id,
            name,
            email,
            coins,
            ..
        } = value;
        Ok(User {
            id: user_id,
            name,
            email,
            coins,
        })
    }
}
