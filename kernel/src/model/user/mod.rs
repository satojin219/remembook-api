use crate::model::id::UserId;

pub mod event;

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub struct BookOwner {
    pub id: UserId,
    pub name: String,
}

#[derive(Debug)]
pub struct CheckoutUser {
    pub id: UserId,
    pub name: String,
}
