use axum::{
    routing::{delete, get, post, put},
    Router,
};
use registry::AppRegistry;

use crate::handler::memo::{
    answer_question, create_memo, delete_memo, get_question, get_question_list, update_memo,
};

pub fn build_memo_routers() -> Router<AppRegistry> {
    let memo_router = Router::new()
        .route("/", post(create_memo))
        .route("/:memo_id", put(update_memo))
        .route("/:memo_id", delete(delete_memo))
        .route("/:book_id/questions", get(get_question_list))
        .route("/:memo_id/question", get(get_question))
        .route("/:memo_id/answer/:question_id", post(answer_question));

    Router::new().nest("/memo", memo_router)
}
