use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::{
    book::{
        event::{CreateBook, DeleteBook, UpdateBook},
        Book,
    },
    id::BookId,
};

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn fetch_all(&self) -> AppResult<Vec<Book>>;
    async fn fetch_by_id(&self, book_id: BookId) -> AppResult<Option<Book>>;
    async fn create_book(&self, event: CreateBook) -> AppResult<()>;
    async fn update_book(&self, event: UpdateBook) -> AppResult<()>;
    async fn delete_book(&self, event: DeleteBook) -> AppResult<()>;
}
