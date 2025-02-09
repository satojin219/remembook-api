use axum::{routing::put, Router};
use registry::AppRegistry;

use crate::handler::user::add_coin;

pub fn build_user_routers() -> Router<AppRegistry> {
    let user_router = Router::new().route("/billing", put(add_coin));
    Router::new().nest("/user", user_router)
}
