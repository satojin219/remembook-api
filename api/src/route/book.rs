use axum::{
    routing::{delete, get, post, put},
    Router,
};
use registry::AppRegistry;

use crate::handler::{
    book::{delete_book, register_book, show_book, show_book_list},
    summary::{answer_question, create_summary, delete_summary, get_question, update_summary},
};

pub fn build_book_routers() -> Router<AppRegistry> {
    let books_router = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/:book_id", get(show_book))
        .route("/:book_id", delete(delete_book));

    let summary_router = Router::new()
        .route("/", post(create_summary))
        .route("/:summary_id", put(update_summary))
        .route("/:summary_id", delete(delete_summary))
        .route("/:summary_id/question", get(get_question))
        .route("/:summary_id/answer/:question_id", post(answer_question));

    Router::new()
        .nest("/books", books_router)
        .nest("/books/:book_id/summary", summary_router)
}
