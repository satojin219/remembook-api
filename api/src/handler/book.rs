use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use garde::Validate;
use kernel::model::{book::event::DeleteBook, id::BookId};
use registry::AppRegistry;
use shared::error::AppResult;

use crate::{
    extractor::AuthorizedUser,
    model::{
        book::{
            BookDetailResponse, BookResponse, BooksResponse, CreateBookRequest, ShowBookResponse,
        },
        question::QuestionResponse,
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
        .create_book(req.into(), user.id())
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn show_book_list(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<BooksResponse>> {
    let books = registry
        .book_repository()
        .get_by_user_id(user.id())
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
) -> AppResult<Json<ShowBookResponse>> {
    let book = registry
        .book_repository()
        .get_by_book_id(book_id)
        .await?
        .unwrap();

    let questions = registry
        .question_repository()
        .get_list_by_book_id(book_id)
        .await?
        .into_iter()
        .map(QuestionResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(ShowBookResponse {
        book: BookDetailResponse::from(book),
        questions,
    }))
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
