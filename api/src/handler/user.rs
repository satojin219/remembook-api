use axum::{extract::State, http::StatusCode, Json};
use garde::Validate;
use kernel::model::user::event::UpdateCoin;
use registry::AppRegistry;
use shared::error::AppResult;

use crate::{
    extractor::AuthorizedUser,
    model::user::{AddCoinRequest, AddCoinRequestWithIds},
};

pub async fn add_coin(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<AddCoinRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;
    let add_coin_request = AddCoinRequestWithIds(user.id(), req.clone());
    registry
        .user_repository()
        .update_coin(add_coin_request.into())
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
