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
        let res: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
        SELECT book_id, title, author, url FROM books
        "#,
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        // BookRow -> Book の変換
        let books = res
            .into_iter()
            .map(|row| row.into_book()) // ここで `From<BookRow> for Book` を実装することを想定
            .collect();

        Ok(books)
    }

    async fn fetch_by_id(&self, book_id: BookId) -> AppResult<Option<Book>> {
        let row: Option<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
        SELECT book_id, title, author, url FROM books WHERE book_id = $1
            "#,
            book_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        match row {
            Some(r) => Ok(Some(r.into_book())),
            None => Ok(None),
        }
    }

    async fn create_book(&self, event: CreateBook) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO books(title, author, url)
                VALUES($1, $2, $3)
            "#,
            event.title,
            event.author,
            event.url
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }
    async fn update_book(&self, event: UpdateBook) -> AppResult<()> {
        let res = sqlx::query!(
            r#"
                UPDATE books
                SET title = $1, author = $2, url = $3
                WHERE book_id = $4
            "#,
            event.title,
            event.author,
            event.url,
            event.book_id as _,
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound("Specified boook not found".into()));
        }

        Ok(())
    }
    async fn delete_book(&self, event: DeleteBook) -> AppResult<()> {
        let res = sqlx::query!(
            r#"
                DELETE FROM books
                WHERE book_id = $1
            "#,
            event.book_id as _,
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound("Specified boook not found".into()));
        }

        Ok(())
    }
}
