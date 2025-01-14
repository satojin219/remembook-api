use axum::{
    routing::{delete, get, post, put},
    Router,
};
use registry::AppRegistry;

use crate::handler::summary::{
    answer_question, create_summary, delete_summary, get_question, get_question_list, update_summary
};

pub fn build_summary_routers() -> Router<AppRegistry> {
    let summary_router = Router::new()
        .route("/", post(create_summary))
        .route("/:summary_id", put(update_summary))
        .route("/:summary_id", delete(delete_summary))
        .route("/:book_id/questions",get(get_question_list))
        .route("/:summary_id/question", get(get_question))
        .route("/:summary_id/answer/:question_id", post(answer_question));

    Router::new().nest("/summary", summary_router)
}
