use crate::model::id::BookId;

#[derive(Debug)]
pub struct CreateBook {
    pub title: String,
    pub url: String,
    pub author: String,
}

#[derive(Debug)]
pub struct UpdateBook {
    pub book_id: BookId,
    pub title: String,
    pub url: String,
    pub author: String,
}

#[derive(Debug)]
pub struct DeleteBook {
    pub book_id: BookId,
}
