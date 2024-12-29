use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use garde::Validate;
use kernel::model::{
    book::event::{CreateBook, DeleteBook},
    id::BookId,
};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::{
    extractor::AuthorizedUser,
    model::book::{
        BookResponse, BooksResponse, CreateBookRequest, UpdateBookRequest, UpdateBookRequestWithIds,
    },
};

pub async fn register_book(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    registry
        .book_repository()
        .create_book(req.into())
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn show_book_list(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<BooksResponse>> {
    let books = registry
        .book_repository()
        .fetch_all()
        .await?
        .into_iter()
        .map(BookResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(BooksResponse { books }))
}

pub async fn show_book(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<BookResponse>> {
    registry
        .book_repository()
        .fetch_by_id(book_id)
        .await
        .and_then(|opt_book| match opt_book {
            Some(book) => Ok(Json(BookResponse::from(book))),
            None => Err(AppError::EntityNotFound("Book not found".to_string())),
        })
}

pub async fn update_book(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateBookRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;
    let update_book = UpdateBookRequestWithIds::new(book_id, req);
    registry
        .book_repository()
        .update_book(update_book.into())
        .await
        .map(|_| StatusCode::OK)
}

pub async fn delete_book(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    registry
        .book_repository()
        .delete_book(DeleteBook { book_id })
        .await
        .map(|_| StatusCode::OK)
}
