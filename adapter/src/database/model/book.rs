use kernel::model::{
    book::{Book, BookDetail},
    id::BookId,
};

pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub author: Vec<String>,
    pub image_url: String,
    pub google_books_id: String,
}

impl BookRow {
    pub fn into_book(self) -> Book {
        Book {
            id: self.book_id,
            title: self.title,
            author: self.author,
            image_url: self.image_url,
            google_books_id: self.google_books_id,
        }
    }
}

pub struct BookDetailRow {
    pub book_id: BookId,
    pub google_books_id: String,
}

impl BookDetailRow {
    pub fn into_book_detail(self) -> BookDetail {
        BookDetail {
            id: self.book_id,
            google_books_id: self.google_books_id,
        }
    }
}
