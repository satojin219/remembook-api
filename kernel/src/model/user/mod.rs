use crate::model::id::UserId;
use chrono::{DateTime, Utc};

pub mod event;

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub coins: i32,
    pub logined_at: DateTime<Utc>,
}
