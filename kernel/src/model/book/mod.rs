use super::id::BookId;

pub mod event;

#[derive(Debug, PartialEq, Eq)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub image_url: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BookDetail {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub image_url: String,
    pub link_url: String,
}
