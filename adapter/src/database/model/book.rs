use kernel::model::{book::Book, id::BookId};
pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub url: String,
    pub author: String,
}

impl BookRow {
    pub fn into_book(self) -> Book {
        Book {
            id: self.book_id,
            title: self.title,
            author: self.author,
            url: self.url,
        }
    }
}
