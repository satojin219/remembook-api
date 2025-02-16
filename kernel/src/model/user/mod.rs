

use crate::model::id::UserId;

pub mod event;

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub coins: i32,
}
