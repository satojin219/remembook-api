use axum::Router;
use registry::AppRegistry;

use super::{book::build_book_routers, memo::build_memo_routers, user::build_user_routers};

pub fn routes() -> Router<AppRegistry> {
    let router = Router::new()
        .merge(build_book_routers())
        .merge(build_memo_routers())
        .merge(build_user_routers());

    Router::new().nest("/api/v1", router)
}
