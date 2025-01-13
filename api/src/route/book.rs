use axum::{
    routing::{delete, get, post},
    Router,
};
use registry::AppRegistry;

use crate::handler::book::{delete_book, register_book, show_book, show_book_list};

pub fn build_book_routers() -> Router<AppRegistry> {
    let books_router = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/:book_id", get(show_book))
        .route("/:book_id", delete(delete_book));

    Router::new().nest("/books", books_router)
}
