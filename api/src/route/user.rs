use axum::{routing::get, Router};
use registry::AppRegistry;

use crate::handler::user::get_me;

pub fn build_user_routers() -> Router<AppRegistry> {
    let user_router = Router::new().route("/me", get(get_me));

    Router::new().nest("/user", user_router)
}
