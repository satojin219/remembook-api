use super::id::BookId;

pub mod event;

#[derive(Debug, PartialEq, Eq)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub author: Vec<String>,
    pub image_url: String,
    pub google_books_id: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BookDetail {
    pub id: BookId,
    pub google_books_id: String,
}
