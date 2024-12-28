use axum::Router;
use registry::AppRegistry;

use super::book::build_book_routers;

pub fn routes() -> Router<AppRegistry> {
    let router = Router::new().merge(build_book_routers());

    Router::new().nest("/api/v1", router)
}
