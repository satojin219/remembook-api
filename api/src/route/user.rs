use axum::{
    routing::{get, put},
    Router,
};
use registry::AppRegistry;

use crate::handler::user::{add_coin, get_me};

pub fn build_user_routers() -> Router<AppRegistry> {
    let user_router = Router::new()
        .route("/charge", put(add_coin))
        .route("/me", get(get_me));

    Router::new().nest("/user", user_router)
}
