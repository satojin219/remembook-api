use crate::model::id::BookId;

#[derive(Debug)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub image_url: String,
    pub link_url: String,
}

#[derive(Debug)]
pub struct DeleteBook {
    pub book_id: BookId,
}
