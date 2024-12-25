use super::id::BookId;

pub mod event;

#[derive(Debug, PartialEq, Eq)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub url: String,
    pub author: String,
}
