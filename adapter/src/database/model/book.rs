use kernel::model::{
    book::{Book, BookDetail},
    id::BookId,
};

pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub image_url: String,
}

impl BookRow {
    pub fn into_book(self) -> Book {
        Book {
            id: self.book_id,
            title: self.title,
            image_url: self.image_url,
        }
    }
}

pub struct BookDetailRow {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub image_url: String,
    pub link_url: String,
}

impl BookDetailRow {
    pub fn into_book_detail(self) -> BookDetail {
        BookDetail {
            id: self.book_id,
            title: self.title,
            author: self.author,
            image_url: self.image_url,
            link_url: self.link_url,
        }
    }
}
