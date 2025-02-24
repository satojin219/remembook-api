use axum::{extract::State, http::StatusCode, Json};
use garde::Validate;
use kernel::model::user::event::{AddPurchaseHistory, UpdateCoin};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::{
    extractor::AuthorizedUser,
    model::user::{AddCoinRequest, UserResponse},
};

// NOTE: add_coinはwebhook上で呼び出されるので、ブラウザ上のaccessTokenを渡すことができない。なのでreqから直接uesr_idを取得する
pub async fn add_coin(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<AddCoinRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    registry
        .user_repository()
        .add_purchase_history(AddPurchaseHistory {
            user_id: user.id(),
            amount: req.amount,
            session_id: req.session_id,
        })
        .await?;

    registry
        .user_repository()
        .update_coin(UpdateCoin {
            user_id: user.id(),
            amount: req.amount,
        })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_me(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<UserResponse>> {
    let user = registry
        .user_repository()
        .find_current_user(user.id())
        .await?
        .ok_or(AppError::EntityNotFound("User not found".into()))?;

    Ok(Json(UserResponse::from(user)))
}
