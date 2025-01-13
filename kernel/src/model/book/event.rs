use crate::model::id::BookId;

#[derive(Debug)]
pub struct CreateBook {
    pub title: String,
    pub author: Vec<String>,
    pub image_url: String,
    pub google_books_id: String,
}

#[derive(Debug)]
pub struct DeleteBook {
    pub book_id: BookId,
}
