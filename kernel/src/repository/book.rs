use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::{
    book::{
        event::{CreateBook, DeleteBook},
        Book, BookDetail,
    },
    id::{BookId, UserId},
};

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn get_by_user_id(&self, user_id: UserId) -> AppResult<Vec<Book>>;
    async fn get_by_book_id(&self, book_id: BookId) -> AppResult<Option<BookDetail>>;
    async fn create_book(&self, event: CreateBook, user_id: UserId) -> AppResult<BookId>;
    async fn delete_book(&self, event: DeleteBook) -> AppResult<()>;
}
