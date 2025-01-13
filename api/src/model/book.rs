use garde::Validate;
use kernel::model::{
    book::{event::CreateBook, Book, BookDetail},
    id::BookId,
};
use serde::{Deserialize, Serialize};

use super::question::QuestionResponse;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(length(min = 1))]
    pub author: Vec<String>,
    #[garde(url)]
    pub image_url: String,
    #[garde(length(min = 1))]
    pub google_books_id: String,
}

impl From<CreateBookRequest> for CreateBook {
    fn from(value: CreateBookRequest) -> Self {
        let CreateBookRequest {
            title,
            author,
            image_url,
            google_books_id,
        } = value;
        Self {
            title,
            author,
            image_url,
            google_books_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub id: BookId,
    pub title: String,
    pub author: Vec<String>,
    pub image_url: String,
    pub google_books_id: String,
}

impl From<Book> for BookResponse {
    fn from(value: Book) -> Self {
        let Book {
            id,
            title,
            author,
            image_url,
            google_books_id,
        } = value;
        Self {
            id,
            title,
            author,
            image_url,
            google_books_id,
        }
    }
}

#[derive(Serialize)]
pub struct BooksResponse {
    pub books: Vec<BookResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookDetailResponse {
    pub id: BookId,
    pub google_books_id: String,
}

impl From<BookDetail> for BookDetailResponse {
    fn from(book_value: BookDetail) -> Self {
        let BookDetail {
            id,
            google_books_id,
        } = book_value;
        Self {
            id,
            google_books_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowBookResponse {
    pub book: BookDetailResponse,
    pub questions: Vec<QuestionResponse>,
}
