use axum::{
    routing::{delete, get, post, put},
    Router,
};
use registry::AppRegistry;

use crate::handler::book::{delete_book, register_book, show_book, show_book_list};

pub fn build_summary_routers() -> Router<AppRegistry> {
    let books_routers = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/:summary_id", get(show_book))
        .route("/:summary_id", delete(delete_book));

    Router::new().nest("/summaries", books_routers)
}
