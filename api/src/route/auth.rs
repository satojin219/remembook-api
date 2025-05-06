use axum::{routing::post, Router};
use registry::AppRegistry;

use crate::handler::auth::{change_email, change_password, login, logout, signup};

pub fn routes() -> Router<AppRegistry> {
    let auth_router = Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/change_email", post(change_email))
        .route("/change_password", post(change_password));
    Router::new().nest("/auth", auth_router)
}
