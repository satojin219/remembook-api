use axum::http::StatusCode;
use axum::{extract::State, Json};
use kernel::model::user::event::DeleteUser;
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::extractor::AuthorizedUser;
use crate::model::user::UserResponse;

pub async fn get_me(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<UserResponse>> {
    let me = registry
        .user_repository()
        .find_current_user(user.id())
        .await?
        .ok_or(AppError::EntityNotFound("User not found".into()))?;

    let valid_coins = registry
        .coin_repository()
        .get_valid_coins_by_user_id(user.id())
        .await?;

    let mut user_response = UserResponse {
        user_id: me.id,
        coins: valid_coins,
        logined_at: me.logined_at.to_rfc3339(),
    };

    Ok(Json(user_response))
}

pub async fn delete_user(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    registry
        .auth_repository()
        .delete_token(&user.access_token)
        .await?;

    registry
        .user_repository()
        .delete(DeleteUser { user_id: user.id() })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
