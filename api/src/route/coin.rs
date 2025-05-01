use axum::{
    routing::{get, post, put},
    Router,
};

use registry::AppRegistry;

use crate::handler::coin::{consume_coin, create_coin, get_purchase_history, get_valid_coins};

pub fn build_coin_routers() -> Router<AppRegistry> {
    let coin_router = Router::new()
        .route("/", post(create_coin))
        .route("/", put(consume_coin))
        .route("/", get(get_valid_coins))
        .route("/history", get(get_purchase_history));

    Router::new().nest("/coin", coin_router)
}
