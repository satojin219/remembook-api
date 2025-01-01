use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        book::{
            event::{CreateBook, DeleteBook, UpdateBook},
            Book,
        },
        id::BookId,
    },
    repository::book::BookRepository,
};
use shared::error::{AppError, AppResult};

use crate::database::{model::book::BookRow, ConnectionPool};

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn fetch_all(&self) -> AppResult<Vec<Book>> {
        todo!()
    }

    async fn fetch_by_id(&self, _book_id: BookId) -> AppResult<Option<Book>> {
        todo!()
    }

    async fn create_book(&self, _event: CreateBook) -> AppResult<()> {
        todo!()
    }

    async fn update_book(&self, _event: UpdateBook) -> AppResult<()> {
        todo!()
    }

    async fn delete_book(&self, _event: DeleteBook) -> AppResult<()> {
        todo!()
    }
}
