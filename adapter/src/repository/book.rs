use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        book::{
            event::{CreateBook, DeleteBook},
            Book, BookDetail,
        },
        id::{BookId, UserId},
    },
    repository::book::BookRepository,
};
use shared::error::{AppError, AppResult};

use crate::database::{
    model::book::{BookDetailRow, BookRow},
    ConnectionPool,
};

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn get_by_user_id(&self, user_id: UserId) -> AppResult<Vec<Book>> {
        let res: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT book_id, title,author, image_url, google_books_id FROM books WHERE user_id = $1
            "#,
            user_id as _
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
    async fn get_by_book_id(&self, book_id: BookId) -> AppResult<Option<BookDetail>> {
        let row: Option<BookDetailRow> = sqlx::query_as!(
            BookDetailRow,
            r#"
        SELECT
            book_id,
            google_books_id
        FROM books
        WHERE book_id = $1
            "#,
            book_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        match row {
            Some(r) => Ok(Some(r.into_book_detail())),
            None => Ok(None),
        }
    }
    async fn create_book(&self, event: CreateBook, user_id: UserId) -> AppResult<BookId> {
        let book_id: BookId = sqlx::query!(
            r#"
            INSERT INTO books(title, author, image_url, google_books_id, user_id)
            VALUES($1, $2, $3, $4, $5)
            RETURNING book_id
        "#,
            event.title,
            &event.author,
            event.image_url,
            event.google_books_id,
            user_id as _
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?
        .book_id
        .into();

        Ok(book_id)
    }
    async fn delete_book(&self, event: DeleteBook) -> AppResult<()> {
        let row = sqlx::query!(
            r#"
                DELETE FROM books
                WHERE book_id = $1
            "#,
            event.book_id as _,
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if row.rows_affected() < 1 {
            return Err(AppError::EntityNotFound("Specified book not found".into()));
        }

        Ok(())
    }
}
