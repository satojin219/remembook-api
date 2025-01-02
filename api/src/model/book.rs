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
    pub author: String,
    #[garde(url)]
    pub image_url: String,
    #[garde(url)]
    pub link_url: String,
}

impl From<CreateBookRequest> for CreateBook {
    fn from(value: CreateBookRequest) -> Self {
        let CreateBookRequest {
            title,
            author,
            image_url,
            link_url,
        } = value;
        Self {
            title,
            author,
            image_url,
            link_url,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub id: BookId,
    pub title: String,
    pub image_url: String,
}

impl From<Book> for BookResponse {
    fn from(value: Book) -> Self {
        let Book {
            id,
            title,
            image_url,
        } = value;
        Self {
            id,
            title,
            image_url,
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
    pub title: String,
    pub author: String,
    pub image_url: String,
    pub link_url: String,
}

impl From<BookDetail> for BookDetailResponse {
    fn from(book_value: BookDetail) -> Self {
        let BookDetail {
            id,
            title,
            image_url,
            author,
            link_url,
        } = book_value;
        Self {
            id,
            title,
            image_url,
            author,
            link_url,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowBookResponse {
    pub book: BookDetailResponse,
    pub questions: Vec<QuestionResponse>,
}
