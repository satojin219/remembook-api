use axum::{
    routing::{delete, get},
    Router,
};
use registry::AppRegistry;

use crate::handler::user::{delete_user, get_me};

pub fn build_user_routers() -> Router<AppRegistry> {
    let user_router = Router::new()
        .route("/me", get(get_me))
        .route("/delete", delete(delete_user));

    Router::new().nest("/user", user_router)
}
