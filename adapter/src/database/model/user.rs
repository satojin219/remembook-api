use chrono::{DateTime, Utc};
use kernel::model::{id::UserId, user::User};
use shared::error::AppError;
use sqlx::types::chrono::{DateTime as SqlxDateTime, Utc as SqlxUtc};

pub struct UserRow {
    pub user_id: UserId,
    pub name: String,
    pub email: String,
    pub coins: i32,
    pub created_at: SqlxDateTime<SqlxUtc>,
    pub updated_at: SqlxDateTime<SqlxUtc>,
    pub logined_at: SqlxDateTime<SqlxUtc>,
}

impl TryFrom<UserRow> for User {
    type Error = AppError;

    fn try_from(value: UserRow) -> Result<Self, Self::Error> {
        let UserRow {
            user_id,
            name,
            email,
            coins,
            logined_at,
            ..
        } = value;

        let logined_at_chrono = DateTime::<Utc>::from_utc(logined_at.naive_utc(), Utc);

        Ok(User {
            id: user_id,
            name,
            email,
            coins,
            logined_at: logined_at_chrono,
        })
    }
}
